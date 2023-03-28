use actix_web::{middleware, App, HttpServer};

mod settings;

extern crate presentation;
extern crate infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conf = settings::Settings::new().unwrap();

    log::info!("Starting HTTP server at {}", conf.web_url);

    infrastructure::configure(&conf.postgres_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .configure(presentation::configure)
            .wrap(middleware::Logger::default())
        })
        .bind(&conf.web_url)?
        .run()
        .await
}
