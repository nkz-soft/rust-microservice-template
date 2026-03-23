use crate::api;
use crate::errors::HttpError;
use actix_web::web;
use actix_web::ResponseError;
use actix_web::{error::InternalError, error::JsonPayloadError, error::QueryPayloadError, Error};

extern crate application;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(json_config());
    cfg.app_data(query_config());
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/auth").service(api::issue_token))
            .service(
                web::scope("/to-do-items")
                    .service(api::get_all)
                    .service(api::create)
                    .service(api::get_by_id)
                    .service(api::update)
                    .service(api::delete),
            )
            .service(
                web::scope("/audit")
                    .service(web::scope("/to-do-items").service(api::get_deleted_by_id_for_audit)),
            )
            .service(
                web::scope("/healthz")
                    .service(api::startup)
                    .service(api::ready)
                    .service(api::live),
            ),
    );
}

fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(8 * 1024)
        .error_handler(|err: JsonPayloadError, _req| map_payload_error(err.into()))
}

fn query_config() -> web::QueryConfig {
    web::QueryConfig::default()
        .error_handler(|err: QueryPayloadError, _req| map_payload_error(err.into()))
}

fn map_payload_error(err: HttpError) -> Error {
    InternalError::from_response(err.to_string(), err.error_response()).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::{CreateToDoItemRequest, GetAllToDoItemsQueryRequest};
    use actix_web::{http::StatusCode, test, web, App, HttpResponse, Result};
    use validator::Validate;

    async fn json_echo(item: web::Json<CreateToDoItemRequest>) -> Result<HttpResponse, HttpError> {
        item.validate()?;
        Ok(HttpResponse::Ok().finish())
    }

    async fn query_echo(
        query: web::Query<GetAllToDoItemsQueryRequest>,
    ) -> Result<HttpResponse, HttpError> {
        query.validate()?;
        query.validate_search().map_err(HttpError::bad_request)?;
        query.validate_sort().map_err(HttpError::bad_request)?;
        Ok(HttpResponse::Ok().finish())
    }

    #[actix_web::test]
    async fn invalid_json_payload_returns_problem_details_400() {
        let app = test::init_service(
            App::new()
                .app_data(json_config())
                .route("/json", web::post().to(json_echo)),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/json")
            .set_json(serde_json::json!({
                "title": "   ",
                "note": "note"
            }))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn oversized_json_payload_returns_problem_details_400() {
        let app = test::init_service(
            App::new()
                .app_data(json_config())
                .route("/json", web::post().to(json_echo)),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/json")
            .insert_header(("content-type", "application/json"))
            .set_payload(format!(
                "{{\"title\":\"{}\",\"note\":\"ok\"}}",
                "a".repeat(9000)
            ))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn invalid_query_payload_returns_problem_details_400() {
        let app = test::init_service(
            App::new()
                .app_data(query_config())
                .route("/query", web::get().to(query_echo)),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/query?page=0&page_size=20&search=test")
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn valid_query_payload_returns_ok() {
        let app = test::init_service(
            App::new()
                .app_data(query_config())
                .route("/query", web::get().to(query_echo)),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/query?page=1&page_size=10&search=todo&sort=title:desc")
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
