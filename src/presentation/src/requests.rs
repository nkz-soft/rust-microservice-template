use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateToDoItemRequest {
    pub title: String,
    pub note: String,
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateToDoItemRequest {
    pub title: String,
    pub note: String,
}
