use actix_web::error::ErrorServiceUnavailable;
use actix_web::web::Data;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use diesel::connection::SimpleConnection;
use infrastructure::DbPool;
use tokio::task;

const OK_STATUS: &str = "Ok";

/// Handles the health check for the application startup.
///
/// This endpoint is used to check if the application is up and running.
/// It returns a response with the current status of the application.
#[get("startup")]
pub async fn startup(_: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(OK_STATUS))
}

/// Handles the health check for the application's live status.
///
/// This endpoint is used to check if the application is currently live and accepting requests.
/// It returns a response with the current status of the application.
#[get("live")]
pub async fn live(_: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(OK_STATUS))
}

/// Handles the health check for the application's ready status.
///
/// This endpoint is used to check if the application is currently ready to handle requests.
/// It returns a response with the current status of the application.
#[get("ready")]
pub async fn ready(pool: Data<DbPool>, _: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    let pool = pool.clone();

    task::spawn_blocking(move || -> Result<(), String> {
        let mut connection = pool
            .get()
            .map_err(|err| format!("failed to acquire database connection: {err}"))?;
        connection
            .batch_execute("SELECT 1;")
            .map_err(|err| format!("failed to execute readiness query: {err}"))?;
        Ok(())
    })
    .await
    .map_err(|err| ErrorServiceUnavailable(format!("readiness task join failure: {err}")))?
    .map_err(ErrorServiceUnavailable)?;

    Ok(HttpResponse::Ok().body(OK_STATUS))
}
