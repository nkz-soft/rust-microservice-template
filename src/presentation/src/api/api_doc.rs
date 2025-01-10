use crate::api::app::__path_create;
use crate::api::app::__path_delete;
use crate::api::app::__path_get_all;
use crate::api::app::__path_get_by_id;
use crate::api::app::__path_update;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    paths(
        get_all,
        create,
        update,
        get_by_id,
        delete
    )
)]
pub struct ApiDoc;
