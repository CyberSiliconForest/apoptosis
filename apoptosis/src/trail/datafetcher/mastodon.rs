use crate::trail::datafetcher::{Paginator, User};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

// Minimal table declaration for Mastodon
diesel::table! {
    accounts(id) {
        id -> BigInt,
        username -> Text,
        private_key -> Text,
        suspended_at -> Timestamp,
    }
}

diesel::table! {
    users(id) {
        id -> BigInt,
        account_id -> BigInt,
        disabled -> Bool,
    }
}

diesel::joinable!(users -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, users,);

// Model definition
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = accounts)]
pub struct MastodonAccounts {
    id: i64,
    username: String,
    private_key: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct MastodonUsers {
    id: i64,
    account_id: i64,
    disabled: bool,
}

// TODO: Test me against database with lots of local user...
pub async fn get_active_users(
    conn: &mut AsyncPgConnection,
    paginator: &Paginator,
) -> Result<Vec<User>, anyhow::Error> {
    // Properly limit the query...
    let users = accounts::table
        .inner_join((users::table))
        .filter(
            users::disabled
                .eq(false)
                .and(accounts::suspended_at.is_null()),
        )
        .select((MastodonAccounts::as_select(), MastodonUsers::as_select()))
        .limit(paginator.limit)
        .offset(paginator.offset)
        .load::<(MastodonAccounts, MastodonUsers)>(conn)
        .await?;

    let results: Vec<User> = users
        .into_iter()
        .map(|(account, user)| User {
            id: account.id.to_string(),
            username: account.username,
            private_key: account.private_key,
        })
        .collect();

    // Should I transport it to another object? hmm...

    Ok(results)
}
