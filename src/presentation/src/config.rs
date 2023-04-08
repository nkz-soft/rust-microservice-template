use actix_web::{web};

use crate::api;
extern crate application;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/to-do-items")
        .service(
            web::resource("")
                .route(web::get().to(api::get_all))
                .route(web::post().to(api::create))
                .route(web::put().to(api::update))
            )
        .service(
            web::scope("/{id}").
                service(
            web::resource("")
                    .route(web::get().to(api::get_by_id))
                    .route(web::delete().to(api::delete))
                )
            )
        );
}
