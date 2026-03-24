extern crate infrastructure;
extern crate presentation;

use anyhow::Result;
use starter::run;
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    run().await?.await?;

    Ok(())
}
