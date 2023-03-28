use tokio_postgres::{NoTls, Error};

use crate::migration;

pub async fn configure(connection: &String) -> Result<(), Error> {
    let (mut client, connection) =
        tokio_postgres::connect(connection, NoTls).await?;

    tokio::spawn(async move {
        connection.await.unwrap();
    });

    migration::migrations::runner()
        .run_async(&mut client)
        .await
        .unwrap();

    Ok::<(), Error>(())
}
