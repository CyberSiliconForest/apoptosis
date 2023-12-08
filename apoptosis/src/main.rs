use crate::types::InstanceType;
use clap::{Parser, Subcommand};

mod caspase;
mod trail;
mod types;

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Convert {
        #[arg(long, help = "Instance type to run Apoptosis on")]
        instance_type: InstanceType,

        #[arg(long, help = "Database URL to run Apoptosis on")]
        database_url: String,
    },
    Destruct {
        #[arg(long, help = "Listen address")]
        listen: String,

        #[arg(long, help = "Request parallel per instance")]
        connection_per_instance: i32,

        #[arg(long, help = "Task runner concurrency")]
        thread_cnt: i32,
    },
}

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Apotosis initialized.");

    //

    Ok(())
}
