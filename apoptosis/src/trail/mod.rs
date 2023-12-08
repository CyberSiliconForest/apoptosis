use crate::types::InstanceType;
use crate::Command;

pub async fn applet_main(instance_type: InstanceType, database_url: String) -> anyhow::Result<()> {
    tracing::info!("Running TRAIL");

    Ok(())
}
