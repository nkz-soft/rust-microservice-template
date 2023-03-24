use actix_web::{web, App, HttpResponse, HttpServer};
use crate::config::AppConfig;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conf : AppConfig = confy::load_path(config::CONFIG_FILE_NAME).unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(&conf.url)?
        .run()
        .await
}
