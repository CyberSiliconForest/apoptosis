use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug)]
pub enum InstanceType {
    Mastodon,
    Misskey,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub actor: String,
    pub object: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub activity: Activity,
    pub private_key: String,
}
