use crate::Command;

const CONCURRENCY_SOFT_LIMIT_PER_INSTANCE: i32 = 20;

pub async fn applet_main(
    listen: String,
    connection_per_instance: i32,
    thread_cnt: i32,
    override_concurrency_limit: bool,
) -> anyhow::Result<()> {
    tracing::info!("Running Caspase");

    // Note: there should be connection_per_instance limitation to prevent
    // Unintentional DoS against Mastodon. ref: https://advisory.silicon.moe/advisory/sif-2023-001/
    // Although apoptosis::caspase will reply 410 as fast as it can, but we are living in the world
    // without FTL communication...
    if connection_per_instance > CONCURRENCY_SOFT_LIMIT_PER_INSTANCE && !override_concurrency_limit {
        eprintln!("Refuse to run caspase: connection_per_instance > {}", CONCURRENCY_SOFT_LIMIT_PER_INSTANCE);
        eprintln!("This limitation is in place because Mastodon is refusing to fix SIF-2023-001");
        eprintln!("See: https://advisory.silicon.moe/advisory/sif-2023-001/ for the detailed information.");

        panic!("Refuse to run. See error message.");
    }

    Ok(())
}
