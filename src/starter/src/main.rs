use actix_web::{middleware, App, HttpServer, web};
use deadpool_postgres::tokio_postgres::NoTls;

mod settings;

extern crate presentation;
extern crate infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = settings::Settings::new().unwrap();

    log::info!("Starting HTTP server at {}", &config.web_url);

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    infrastructure::configure(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(presentation::configure)
            .wrap(middleware::Logger::default())
        })
        .bind(&config.web_url)?
        .run()
        .await
}
