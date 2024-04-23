extern crate infrastructure;
extern crate presentation;

use starter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await?.await
}
