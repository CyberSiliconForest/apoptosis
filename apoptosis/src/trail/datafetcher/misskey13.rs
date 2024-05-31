use crate::trail::datafetcher::{Instance, Paginator, User};
use diesel::prelude::*;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures_util::Stream;

table! {
    user(id) {
        id -> Text,
        host -> Nullable<Text>,
        username -> Text,
        isDeleted -> Bool,
        isSuspended -> Bool,
        sharedInbox -> Nullable<Text>,
    }
}

table! {
    user_keypair(userId) {
        userId -> Text,
        privateKey -> Text,
    }
}

joinable!(user_keypair -> user(userId));

allow_tables_to_appear_in_same_query!(user, user_keypair,);

// Model definition
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = user)]
pub struct Misskey13User {
    id: String,
    username: String,
    #[diesel(column_name = "isDeleted")]
    is_deleted: bool,
    #[diesel(column_name = "isSuspended")]
    is_suspended: bool,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = user_keypair)]
pub struct Misskey13KeyPair {
    #[diesel(column_name = "userId")]
    user_id: String,
    #[diesel(column_name = "privateKey")]
    private_key: String,
}

pub async fn get_active_users(
    conn: &mut AsyncPgConnection,
    paginator: &Paginator,
) -> Result<Vec<User>, anyhow::Error> {
    let users = user::table
        .inner_join(user_keypair::table)
        .filter(user::host.is_null())
        .filter(user::isDeleted.eq(false))
        .select((Misskey13User::as_select(), Misskey13KeyPair::as_select()))
        .order_by(user::id.asc())
        .limit(paginator.limit)
        .offset(paginator.offset * paginator.limit)
        .load::<(Misskey13User, Misskey13KeyPair)>(conn)
        .await?;

    let results: Vec<User> = users
        .into_iter()
        .map(|(user, kp)| User {
            id: user.id,
            username: user.username,
            private_key: kp.private_key,
        })
        .collect();

    Ok(results)
}

pub async fn get_shared_inboxes(
    conn: &mut AsyncPgConnection,
    paginator: &Paginator,
) -> Result<Vec<Instance>, anyhow::Error> {
    let instances = user::table
        .filter(user::sharedInbox.is_not_null())
        .order_by(user::sharedInbox.asc())
        .distinct_on(user::sharedInbox)
        .select(user::sharedInbox)
        .limit(paginator.limit)
        .offset(paginator.offset * paginator.limit)
        .load::<Option<String>>(conn)
        .await?;

    let results: Vec<Instance> = instances
        .into_iter()
        .map(|url| Instance {
            shared_inbox: url.unwrap(), // Queried is_not_null so 100% sure to not have None.
            is_alive: true,             // FIXME: properly query it.
        })
        .collect();

    Ok(results)
}
