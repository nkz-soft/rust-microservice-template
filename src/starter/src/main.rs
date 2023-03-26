use actix_web::{middleware, App, HttpServer};
use crate::config::AppConfig;

extern crate presentation;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conf : AppConfig = confy::load_path(config::CONFIG_FILE_NAME).unwrap();

    log::info!("Starting HTTP server at {}", conf.url);

    HttpServer::new(|| {
        App::new()
            .configure(presentation::config::configure)
            .wrap(middleware::Logger::default())
        })
        .bind(&conf.url)?
        .run()
        .await
}
