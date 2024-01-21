pub mod mastodon;
pub mod misskey13;

#[derive(Clone, Debug)]
pub struct User {
    /// User's unique identifier. Misskey will use bizarre aid/aidx
    /// and Mastodon will use timeflake for it
    pub id: String,
    /// Username, of course.
    pub username: String,
    /// PEM encoded RSA-2048 private key block. As most Fediverse instance uses
    pub private_key: String,
}

#[derive(Clone, Debug)]
pub struct Paginator {
    pub limit: i64,
    pub offset: i64,
}

impl User {
    //
}
