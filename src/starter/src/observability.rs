use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::middleware::Next;
use actix_web::Error;
use anyhow::{anyhow, Context, Result};
use application::Settings;
use metrics::{counter, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;
use std::time::Instant;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

static TRACING_INIT: OnceLock<()> = OnceLock::new();
static PROMETHEUS_HANDLE: OnceLock<Result<PrometheusHandle, String>> = OnceLock::new();

#[derive(Clone)]
pub struct ObservabilityConfig {
    pub request_id_header_name: HeaderName,
    pub request_id_response_name: HeaderName,
    pub metrics_enabled: bool,
    pub metrics_path: String,
}

impl ObservabilityConfig {
    pub fn from_settings(settings: &Settings) -> Result<Self> {
        let request_header = settings.observability.request_id_header.trim();
        let request_id_header_name = HeaderName::try_from(request_header)
            .with_context(|| format!("invalid request id header name: {request_header}"))?;

        Ok(Self {
            request_id_header_name: request_id_header_name.clone(),
            request_id_response_name: request_id_header_name,
            metrics_enabled: settings.observability.metrics_enabled,
            metrics_path: settings.observability.metrics_path.clone(),
        })
    }
}

pub fn init_tracing(settings: &Settings) -> Result<()> {
    if TRACING_INIT.get().is_some() {
        return Ok(());
    }

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(settings.observability.log_level.clone()))
        .context("invalid tracing filter configuration")?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .json()
        .try_init()
        .map_err(|err| anyhow!("failed to initialize tracing subscriber: {err}"))?;

    let _ = TRACING_INIT.set(());
    Ok(())
}

pub fn init_prometheus_recorder() -> Result<PrometheusHandle> {
    let result = PROMETHEUS_HANDLE.get_or_init(|| {
        PrometheusBuilder::new()
            .install_recorder()
            .map_err(|err| err.to_string())
    });

    match result {
        Ok(handle) => Ok(handle.clone()),
        Err(err) => Err(anyhow!("failed to initialize prometheus recorder: {err}")),
    }
}

pub async fn observability_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = request
        .app_data::<actix_web::web::Data<ObservabilityConfig>>()
        .cloned()
        .ok_or_else(|| {
            actix_web::error::ErrorInternalServerError("missing observability config")
        })?;

    let start = Instant::now();
    let request_id = extract_or_generate_request_id(&request, &config.request_id_header_name);
    let request_path = request.path().to_string();

    let mut response = next.call(request).await?;
    let status = response.status();

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response
            .headers_mut()
            .insert(config.request_id_response_name.clone(), value);
    }

    let route = response
        .request()
        .match_pattern()
        .unwrap_or_else(|| request_path.clone());
    let method = response.request().method().as_str().to_string();
    let status_class = status_class(status.as_u16());
    let duration_seconds = start.elapsed().as_secs_f64();
    let is_metrics_endpoint = request_path == config.metrics_path;

    if config.metrics_enabled && !is_metrics_endpoint {
        counter!(
            "http_requests_total",
            "method" => method.clone(),
            "route" => route.clone(),
            "status_class" => status_class.to_string()
        )
        .increment(1);
        histogram!(
            "http_request_duration_seconds",
            "method" => method.clone(),
            "route" => route.clone(),
            "status_class" => status_class.to_string()
        )
        .record(duration_seconds);
        if status.is_client_error() || status.is_server_error() {
            counter!(
                "http_request_errors_total",
                "method" => method.clone(),
                "route" => route.clone(),
                "status_class" => status_class.to_string()
            )
            .increment(1);
        }
    }

    info!(
        request_id = %request_id,
        method = %method,
        route = %route,
        status = status.as_u16(),
        duration_ms = start.elapsed().as_millis(),
        "http request completed"
    );

    Ok(response)
}
pub fn extract_or_generate_request_id(
    request: &ServiceRequest,
    request_id_header_name: &HeaderName,
) -> String {
    request
        .headers()
        .get(request_id_header_name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

fn status_class(code: u16) -> &'static str {
    match code {
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::{web, App, HttpResponse};

    #[actix_web::test]
    async fn extract_or_generate_request_id_preserves_header_value() {
        let request = test::TestRequest::default()
            .insert_header(("x-request-id", "abc-123"))
            .to_srv_request();

        let request_id =
            extract_or_generate_request_id(&request, &HeaderName::from_static("x-request-id"));
        assert_eq!(request_id, "abc-123");
    }

    #[actix_web::test]
    async fn extract_or_generate_request_id_generates_for_blank_or_missing() {
        let missing = test::TestRequest::default().to_srv_request();
        let missing_id =
            extract_or_generate_request_id(&missing, &HeaderName::from_static("x-request-id"));
        assert!(!missing_id.is_empty());

        let blank = test::TestRequest::default()
            .insert_header(("x-request-id", " "))
            .to_srv_request();
        let blank_id =
            extract_or_generate_request_id(&blank, &HeaderName::from_static("x-request-id"));
        assert!(!blank_id.is_empty());
        assert_ne!(blank_id, " ");
    }

    #[actix_web::test]
    async fn middleware_sets_response_request_id_header() {
        let config = ObservabilityConfig {
            request_id_header_name: HeaderName::from_static("x-request-id"),
            request_id_response_name: HeaderName::from_static("x-request-id"),
            metrics_enabled: false,
            metrics_path: "/metrics".to_string(),
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(config))
                .wrap(actix_web::middleware::from_fn(observability_middleware))
                .route(
                    "/ok",
                    web::get().to(|| async { HttpResponse::Ok().finish() }),
                ),
        )
        .await;

        let request = test::TestRequest::get().uri("/ok").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.headers().contains_key("x-request-id"));
    }
}
