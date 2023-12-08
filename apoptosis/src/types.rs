use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum InstanceType {
    Mastodon,
    Misskey,
}
