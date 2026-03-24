use actix_web::{get, web::Data, HttpResponse, Result};
use metrics_exporter_prometheus::PrometheusHandle;

/// Exposes Prometheus-compatible runtime metrics.
#[utoipa::path(
    context_path = "",
    tag = "todo",
    responses(
        (status = 200, description = "Prometheus metrics output")
    )
)]
#[get("/metrics")]
pub async fn metrics(handle: Data<PrometheusHandle>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(handle.render()))
}
