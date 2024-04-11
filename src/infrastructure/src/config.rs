use deadpool_postgres::Pool;
use std::ops::DerefMut;
use tokio_postgres::Error;

use crate::migration;

pub async fn configure(pool: &Pool) -> Result<(), Error> {
    let mut obj = pool.get().await.unwrap();
    let client = obj.deref_mut().deref_mut();

    migration::migrations::runner()
        .run_async(client)
        .await
        .unwrap();

    Ok::<(), Error>(())
}
