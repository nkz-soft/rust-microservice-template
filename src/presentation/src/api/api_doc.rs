use crate::api::api_metrics::__path_metrics;
use crate::api::app::__path_create;
use crate::api::app::__path_delete;
use crate::api::app::__path_get_all;
use crate::api::app::__path_get_by_id;
use crate::api::app::__path_get_deleted_by_id_for_audit;
use crate::api::app::__path_update;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Microservice Template API",
        version = "v1"
    ),
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    paths(
        get_all,
        create,
        update,
        get_by_id,
        delete,
        get_deleted_by_id_for_audit,
        metrics
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::ApiDoc;
    use serde_json::Value;
    use utoipa::OpenApi;

    #[test]
    fn openapi_documents_search_parameter_on_todo_list() {
        let openapi = ApiDoc::openapi();
        let openapi_json = serde_json::to_value(&openapi).expect("OpenAPI should serialize");
        let search_parameter = openapi_json["paths"]["/api/v1/to-do-items"]["get"]["parameters"]
            .as_array()
            .expect("parameters should be an array")
            .iter()
            .find(|parameter| parameter["name"] == Value::String("search".into()))
            .expect("search parameter should be documented");

        let description = search_parameter["description"]
            .as_str()
            .expect("search parameter should have a description");
        assert!(description.contains("title and note"));
        assert!(description.contains("Blank values are rejected"));
    }

    #[test]
    fn openapi_documents_request_id_response_description() {
        let openapi = ApiDoc::openapi();
        let openapi_json = serde_json::to_value(&openapi).expect("OpenAPI should serialize");
        let description = openapi_json["paths"]["/api/v1/to-do-items"]["get"]["responses"]["200"]
            ["description"]
            .as_str()
            .expect("response description should exist");

        assert!(description.contains("X-Request-Id"));
    }

    #[test]
    fn openapi_includes_metrics_endpoint() {
        let openapi = ApiDoc::openapi();
        let openapi_json = serde_json::to_value(&openapi).expect("OpenAPI should serialize");
        assert_eq!(
            openapi_json["paths"]["/metrics"]["get"]["summary"],
            Value::String("Exposes Prometheus-compatible runtime metrics.".to_string())
        );
    }
}
