use crate::types::InstanceType;
use clap::{Parser, Subcommand};

mod caspase;
mod cytochrome;
mod mhc;
mod types;
mod util;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Convert {
        #[arg(long, help = "Instance type to run Apoptosis on")]
        instance_type: InstanceType,

        #[arg(long, help = "Database URL to run Apoptosis on")]
        database_url: String,

        #[arg(long, help = "Instance base URL. e.g. https://mastodon.social/")]
        instance_base_url: String,
    },
    Serve {
        #[arg(long, help = "Listen address")]
        listen: String,
    },
    Destruct {
        #[arg(long, help = "Request parallel per instance")]
        connection_per_instance: i32,

        #[arg(long, help = "Task runner concurrency")]
        thread_cnt: i32,

        #[arg(
            long = "override-concurrency-limit-i-know-what-is-sif-2023-001",
            help = "Override concurrency limit. DO NOT ENABLE THIS UNLESS YOU KNOW THE CONSQUENCES."
        )]
        override_concurrency_limit: bool,
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

    tracing::info!("Apoptosis initialized.");

    match args.command {
        Command::Convert {
            instance_type,
            database_url,
            instance_base_url,
        } => {
            cytochrome::applet_main(instance_type, database_url, instance_base_url).await?;
        }
        Command::Serve { listen } => {
            mhc::applet_main(listen).await?;
        }
        Command::Destruct {
            connection_per_instance,
            thread_cnt,
            override_concurrency_limit,
        } => {
            caspase::applet_main(
                connection_per_instance,
                thread_cnt,
                override_concurrency_limit,
            )
            .await?;
        }
    }

    Ok(())
}
