use crate::trail::datafetcher::{Instance, Paginator, User};
use diesel::prelude::*;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures_util::Stream;

table! {
    user(id) {
        id -> Text,
        username -> Text,
        isDeleted -> Bool,
        isSuspended -> Bool,
    }
}

table! {
    user_keypair(userId) {
        userId -> Text,
        privateKey -> Text,
    }
}

table! {
    instance(id) {
        id -> Text,
        host -> Text,
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

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = instance)]
pub struct Misskey13Instances {
    #[diesel(column_name = "host")]
    host: String,
}

pub async fn get_active_users(
    conn: &mut AsyncPgConnection,
    paginator: &Paginator,
) -> Result<Vec<User>, anyhow::Error> {
    let users = user::table
        .inner_join(user_keypair::table)
        .filter(user::isDeleted.eq(false))
        .select((Misskey13User::as_select(), Misskey13KeyPair::as_select()))
        .limit(paginator.limit)
        .offset(paginator.offset)
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

pub async fn get_federated_instances(
    conn: &mut AsyncPgConnection,
    paginator: &Paginator,
) -> Result<Vec<Instance>, anyhow::Error> {
    let instances = instance::table
        .select(Misskey13Instances::as_select())
        .limit(paginator.limit)
        .offset(paginator.offset)
        .load::<Misskey13Instances>(conn)
        .await?;

    let results: Vec<Instance> = instances
        .into_iter()
        .map(|instance| Instance {
            host: instance.host,
            is_alive: true, // FIXME: properly query it.
        })
        .collect();

    Ok(results)
}
