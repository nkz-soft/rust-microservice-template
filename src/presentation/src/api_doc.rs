use crate::api;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    paths(
        api::get_all,
        api::create,
        api::update,
        api::get_by_id,
        api::delete
    )
)]
pub struct ApiDoc;
