use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use deadpool_postgres::tokio_postgres::NoTls;
use log::{debug, info};

mod settings;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    Ok(run_internal(&settings).await?)
}

pub async fn run_with_config(path: &str) -> Result<Server> {
    let settings = Settings::with_path(path).load()?;
    Ok(run_internal(&settings).await?)
}
async fn run_internal(settings: &Settings) -> Result<Server> {
    info!("Starting HTTP server at {}", &settings.service.http_url);
    debug!("with configuration: {:?}", &settings);

    let pool = settings.database.pg.create_pool(None, NoTls)?;

    infrastructure::configure(&pool).await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(presentation::configure)
    })
    .bind(&settings.service.http_url)?
    .run();

    Ok(server)
}
