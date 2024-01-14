use diesel::prelude::*;

diesel::table! {
    user(id) {
        id -> Text,
        username -> Text,
    }
}

diesel::table! {
    user_keypair(userId) {
        userId -> Text,
        privateKey -> Text,
    }
}

diesel::joinable!(user_keypair -> user(userId));

diesel::allow_tables_to_appear_in_same_query!(
    user,
    user_keypair,
);

//const USER_FETCH_QUERY: &str = """
//SELECT
//  "user".id AS id,
//  "user".username AS username,
//  "user_keypair"."privateKey" AS private_key
//FROM "user"
//INNER JOIN "user_keypair" ON "user_keypair"."userId" = "user".id
//WHERE "user"."isDeleted" IS FALSE
//  AND "user"."isSuspended" IS FALSE
//ORDER BY "user".id
//LIMIT ? OFFSET ?;
//"""

//