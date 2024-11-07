use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use deadpool_postgres::tokio_postgres::NoTls;
use log::{debug, info};
use anyhow::Result;

mod settings;

pub async fn run() -> Result<()> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await?.await?;
    Ok(())
}

pub async fn run_with_config(path: &str) -> Result<()> {
    let settings = Settings::with_path(path).load()?;
    run_internal(&settings).await?.await?;
    Ok(())
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
