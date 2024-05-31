mod datafetcher;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use crate::cytochrome::datafetcher::{mastodon, misskey13, Instance};
use crate::cytochrome::datafetcher::{Paginator, User};
use crate::types::{Activity, ApPrivateKey, InstanceType, Payload};

async fn get_active_users(
    pool: &Pool<AsyncPgConnection>,
    instance_type: &InstanceType,
    paginator: Paginator,
) -> Result<Vec<User>, anyhow::Error> {
    let mut conn = pool.get().await.unwrap();
    Ok(match instance_type {
        InstanceType::Mastodon => mastodon::get_active_users(&mut conn, &paginator).await?,
        InstanceType::Misskey => misskey13::get_active_users(&mut conn, &paginator).await?,
    })
}

async fn get_shared_inboxes(
    pool: &Pool<AsyncPgConnection>,
    instance_type: &InstanceType,
    paginator: Paginator,
) -> Result<Vec<Instance>, anyhow::Error> {
    let mut conn = pool.get().await.unwrap();
    Ok(match instance_type {
        InstanceType::Mastodon => mastodon::get_shared_inboxes(&mut conn, &paginator).await?,
        InstanceType::Misskey => misskey13::get_shared_inboxes(&mut conn, &paginator).await?,
    })
}

pub async fn applet_main(
    instance_type: InstanceType,
    database_url: String,
    instance_base_url: String,
) -> anyhow::Result<()> {
    tracing::info!("Running apoptosis::cytochrome applet.");

    let config = ManagerConfig::default();

    let mgr =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);

    let pool = Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .build(mgr)
        .await?;

    // Fetch users till to end...
    // For now, just print it out.

    let mut paginator = Paginator {
        limit: 100,
        offset: 0,
    };

    let mut user_cnt: usize = 0;
    let mut inbox_cnt: usize = 0;

    let mut payloads: Vec<Payload> = Vec::new();
    let mut inboxes: Vec<String> = Vec::new();

    loop {
        // Since we are dealing with offline or read-only instance, we don't need to deal with transaction
        // because there will be no writer exists.
        let active_users = get_active_users(&pool, &instance_type, paginator.clone())
            .await
            .unwrap();

        if active_users.is_empty() {
            break;
        }

        // process active_users
        user_cnt += active_users.len();
        payloads.extend(active_users.into_iter().map(|user| {
            let actor = format!("{}/users/{}", instance_base_url, user.id);
            let activity = Activity {
                context: "https://www.w3.org/ns/activitystreams".into(),
                id: format!("{}/{}", instance_base_url, Uuid::new_v4()),
                activity_type: "Delete".into(),
                actor: actor.clone(),
                object: actor.clone(),
            };

            Payload {
                activity,
                private_key: ApPrivateKey {
                    pem: user.private_key,
                    key_id: format!("{}#main-key", actor),
                },
            }
        }));

        paginator.offset += 1;
    }
    tracing::info!("Total users: {}", user_cnt);

    loop {
        // Since we are dealing with offline or read-only instance, we don't need to deal with transaction
        // because there will be no writer exists.
        let instances = get_shared_inboxes(&pool, &instance_type, paginator.clone())
            .await
            .unwrap();

        if instances.is_empty() {
            break;
        }

        inbox_cnt += instances.len();
        inboxes.extend(
            instances
                .into_iter()
                .filter(|instance| instance.is_alive)
                .map(|instance| instance.shared_inbox),
        );

        paginator.offset += 1;
    }

    tracing::info!("Total shared inboxes: {}", inbox_cnt);

    // Now we have all the payloads and inboxes, we can start to send the payloads to the inboxes.
    // For now, just print it out.
    tracing::info!("Payloads: {:?}", payloads);
    tracing::info!("Inboxes: {:?}", inboxes);

    std::fs::write("./payloads.json", serde_json::to_string(&payloads).unwrap()).unwrap();
    std::fs::write("./inboxes.json", serde_json::to_string(&inboxes).unwrap()).unwrap();

    Ok(())
}
