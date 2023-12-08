use crate::Command;

pub async fn applet_main(
    listen: String,
    connection_per_instance: i32,
    thread_cnt: i32,
) -> anyhow::Result<()> {
    tracing::info!("Running Caspase");

    Ok(())
}
