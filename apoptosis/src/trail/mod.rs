mod datafetcher;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use futures_util::FutureExt;

use crate::types::InstanceType;
use crate::Command;

pub async fn applet_main(instance_type: InstanceType, database_url: String) -> anyhow::Result<()> {
    tracing::info!("Running TRAIL");

    let config = ManagerConfig::default();

    let mgr =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);

    let pool = Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .build(mgr)
        .await?;

    let conn = pool.get().await?;

    // Now, we have a pool.
    // Pass the pool to proper child, to fetch the db.

    Ok(())
}
