use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use deadpool_postgres::tokio_postgres::NoTls;

mod settings;

pub async fn run() -> Result<Server, std::io::Error> {
    let _env_logger = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));
    let settings = Settings::new().unwrap();
    run_internal(&settings).await
}

pub async fn run_with_config(path: &str) -> Result<Server, std::io::Error> {
    let _env_logger = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));
    let settings = Settings::with_path(path).unwrap();
    run_internal(&settings).await
}

pub async fn run_internal(settings: &Settings) -> Result<Server, std::io::Error> {
    log::info!("Starting HTTP server at {}", &settings.web_url);

    let pool = settings.pg.create_pool(None, NoTls).unwrap();

    infrastructure::configure(&pool).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(presentation::configure)
    })
    .bind(&settings.web_url)?
    .run();

    Ok(server)
}
