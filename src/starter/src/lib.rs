use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use application::{AuthService, Settings, ToDoItemService};
use infrastructure::PostgresToDoItemRepository;
use log::{debug, info};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await
}

pub async fn run_with_config(path: &str) -> Result<Server> {
    let settings = Settings::with_path(path).load()?;
    run_internal(&settings).await
}

async fn run_internal(settings: &Settings) -> Result<Server> {
    info!("Starting HTTP server at {}", &settings.service.http_url);
    debug!("with configuration: {:?}", &settings);

    let pool = infrastructure::configure(settings).await?;

    // Create repository with Arc for thread safety
    let repository = Arc::new(PostgresToDoItemRepository::new(&web::Data::new(
        pool.clone(),
    )));

    // Create service with dependency injection
    let todo_service = ToDoItemService::new(repository);
    let auth_service =
        AuthService::new(settings.auth.clone()).map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let server = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(presentation::ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .map(|app| app.configure(presentation::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
                    .url("/api/v1/api-docs/openapi.json", api)
            })
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(todo_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .into_app()
    })
    .bind(&settings.service.http_url)?
    .run();

    Ok(server)
}
