use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection};

// Minimal table declaration for Mastodon
diesel::table! {
    accounts(id) {
        id -> BigInt,
        username -> Text,
        private_key -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    users,
);

//const USER_FETCH_QUERY: &str = """
//SELECT
//  accounts.id AS id,
//  accounts.username AS username,
//  accounts.private_key AS private_key
//FROM accounts
//INNER JOIN users ON accounts.id = users.account_id
//WHERE users.disabled IS NOT TRUE
//  AND accounts.suspended_at IS NULL
//ORDER BY accounts.id
//LIMIT ? OFFSET ?;
//""";

