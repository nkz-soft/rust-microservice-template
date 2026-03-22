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
        get_deleted_by_id_for_audit
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
}
