use actix_web::{get, Error, HttpRequest, HttpResponse};

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
pub async fn ready(_: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(OK_STATUS))
}
