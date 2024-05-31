use task_exec_queue::SpawnExt;
use url::Url;

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

    let (exec, task_runner) = task_exec_queue::Builder::default()
        .workers(thread_cnt as usize)
        .queue_max(128)
        .build();
    let root_fut = async move {
        tokio::spawn(async {
            //start executor
            task_runner.await;
        });

        for inbox in inboxes {
            let inbox = Url::parse(&inbox)?;
            for payload in &payloads {
                let inbox = inbox.clone();
                let payload = payload.clone();
                async move {
                    let requester = SignedRequester::new(
                        &payload.private_key.pem,
                        &payload.private_key.key_id,
                        None,
                    );
                    tracing::info!(
                        "Sending payload: {:?} to {}",
                        payload.activity,
                        inbox.to_string()
                    );
                    requester
                        .post(&inbox, serde_json::to_value(&payload.activity)?)
                        .await?;
                    anyhow::Ok(())
                }
                .spawn(&exec)
                .await
                .map_err(|e| {
                    eprintln!("Error: Something terrible");
                })
                .unwrap();
            }
        }

        loop {
            if exec.waiting_count() == 0 {
                tracing::info!("Waiting sz = 0");
                if exec.active_count() == 0 {
                    tracing::info!("Running sz = 0");
                    break;
                }
            }
            tracing::info!(
                "exec.actives: {}, waitings: {}",
                exec.active_count(),
                exec.waiting_count()
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        //exec.flush().await?;
        tracing::info!("All tasks are done");

        anyhow::Ok(())
    };

    root_fut.await?;

    Ok(())
}
