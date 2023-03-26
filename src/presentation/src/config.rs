use actix_web::{http, web};
use serde_json;
extern crate application;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/get").to(|| async
        {
            let get_handler = application::handlers::GetToDoItemQueryHandler::new();
            let data = get_handler.execute();
            actix_web::HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .body(serde_json::to_string(&data).unwrap())
        }));
}
