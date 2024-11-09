use actix_web::web;

use crate::api;
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
}
