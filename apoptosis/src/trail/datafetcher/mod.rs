mod mastodon;
mod misskey13;

pub struct User {
    /// User's unique identifier. Misskey will use bizarre aid/aidx
    /// and Mastodon will use timeflake for it
    id: String,
    /// Username, of course.
    username: String,
    /// PEM encoded RSA-2048 private key block. As most Fediverse instance uses
    private_key: String,
}

pub struct Paginator {
    limit: i64,
    offset: i64,
}

impl User {
    //
}
