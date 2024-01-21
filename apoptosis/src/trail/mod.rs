mod datafetcher;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use futures_util::FutureExt;

use crate::trail::datafetcher::{mastodon, misskey13};
use crate::trail::datafetcher::{Paginator, User};
use crate::types::InstanceType;
use crate::Command;

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

pub async fn applet_main(instance_type: InstanceType, database_url: String) -> anyhow::Result<()> {
    tracing::info!("Running TRAIL");

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

    loop {
        // Since we are dealing with offline or read-only instance, we don't need to deal with transaction
        // because there will be no writer exists.
        let active_users = get_active_users(&pool, &instance_type, paginator.clone())
            .await
            .unwrap();

        if active_users.is_empty() {
            break;
        }

        println!("{:?}", active_users);

        paginator.offset += 1;
    }

    Ok(())
}
