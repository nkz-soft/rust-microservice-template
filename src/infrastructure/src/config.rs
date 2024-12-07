use deadpool_postgres::Pool;
use std::ops::DerefMut;

use crate::migration;

pub async fn configure(pool: &Pool) -> anyhow::Result<()> {
    let mut obj = pool.get().await?;
    let client = obj.deref_mut().deref_mut();

    migration::migrations::runner().run_async(client).await?;

    Ok(())
}
