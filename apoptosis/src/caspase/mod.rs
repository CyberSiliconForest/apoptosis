use crate::types::Payload;
use crate::util::requester::SignedRequester;
use crate::Command;

const CONCURRENCY_SOFT_LIMIT_PER_INSTANCE: i32 = 20;

pub async fn applet_main(
    connection_per_instance: i32,
    thread_cnt: i32,
    override_concurrency_limit: bool,
) -> anyhow::Result<()> {
    tracing::info!("Running apoptosis::caspase applet");

    // Note: there should be connection_per_instance limitation to prevent
    // Unintentional DoS against Mastodon. ref: https://advisory.silicon.moe/advisory/sif-2023-001/
    // Although apoptosis::mhc will reply 410 as fast as it can, but we are living in the world
    // without FTL communication...
    if connection_per_instance > CONCURRENCY_SOFT_LIMIT_PER_INSTANCE && !override_concurrency_limit
    {
        eprintln!(
            "Refuse to run caspase: connection_per_instance > {}",
            CONCURRENCY_SOFT_LIMIT_PER_INSTANCE
        );
        eprintln!("This limitation is in place because Mastodon is refusing to fix SIF-2023-001");
        eprintln!("See: https://advisory.silicon.moe/advisory/sif-2023-001/ for the detailed information.");

        panic!("Refuse to run. See error message.");
    }

    // Load data from the file
    // TODO: Replace it with the database

    let payloads: Vec<Payload> =
        serde_json::from_str(&std::fs::read_to_string("./payloads.json")?)?;
    let inboxes: Vec<String> = serde_json::from_str(&std::fs::read_to_string("./inboxes.json")?)?;

    // TODO: Parallelize the task

    for inbox in inboxes {
        for payload in &payloads {
            //SignedRequester::new(pem, key_id, user_agent)
        }
    }

    Ok(())
}
