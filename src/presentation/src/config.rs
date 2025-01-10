use crate::api;
use actix_web::web;

extern crate application;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/to-do-items")
            .service(api::get_all)
            .service(api::create)
            .service(api::get_by_id)
            .service(api::update)
            .service(api::delete),
    );
    cfg.service(
        web::scope("/healthz")
            .service(api::startup)
            .service(api::ready)
            .service(api::live),
    );
}
