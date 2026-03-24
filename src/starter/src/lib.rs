mod observability;

use actix_web::dev::Server;
use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use application::{Settings, ToDoItemService};
use infrastructure::PostgresToDoItemRepository;
use std::sync::Arc;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;
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
    observability::init_tracing(settings)?;
    let observability_config = observability::ObservabilityConfig::from_settings(settings)?;
    let prometheus_handle = observability::init_prometheus_recorder()?;

    info!("Starting HTTP server at {}", &settings.service.http_url);
    debug!("with configuration: {:?}", &settings);

    let pool = infrastructure::configure(settings).await?;

    // Create repository with Arc for thread safety
    let repository = Arc::new(PostgresToDoItemRepository::new(&web::Data::new(
        pool.clone(),
    )));

    // Create service with dependency injection
    let todo_service = ToDoItemService::new(repository);
    let audit_settings = settings.audit.clone();
    let observability_settings = observability_config.clone();
    let metrics_handle = prometheus_handle.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(observability_settings.clone()))
            .app_data(web::Data::new(metrics_handle.clone()))
            .into_utoipa_app()
            .openapi(presentation::ApiDoc::openapi())
            .map(|app| app.wrap(TracingLogger::default()))
            .map(|app| app.wrap(from_fn(observability::observability_middleware)))
            .map(|app| app.configure(presentation::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
                    .url("/api/v1/api-docs/openapi.json", api)
            })
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(todo_service.clone()))
            .app_data(web::Data::new(audit_settings.clone()))
            .into_app()
    })
    .bind(&settings.service.http_url)?
    .run();

    Ok(server)
}
