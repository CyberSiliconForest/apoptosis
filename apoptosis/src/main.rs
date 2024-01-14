use crate::types::InstanceType;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection};

mod caspase;
mod trail;
mod types;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
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

#[derive(Parser, Clone, Debug)] //
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    tracing::info!("Apotosis initialized.");

    match args.command {
        Command::Convert {
            instance_type,
            database_url,
        } => {
            trail::applet_main(instance_type, database_url).await?;
        }
        Command::Destruct {
            listen,
            connection_per_instance,
            thread_cnt,
        } => {
            caspase::applet_main(listen, connection_per_instance, thread_cnt).await?;
        }
    }

    Ok(())
}
