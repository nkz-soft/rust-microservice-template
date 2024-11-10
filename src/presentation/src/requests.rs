use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateToDoItemRequest {
    /// The title of the to-do item
    pub title: String,
    /// The note of the to-do item
    pub note: String,
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateToDoItemRequest {
    /// The title of the to-do item
    pub title: String,
    /// The note of the to-do item
    pub note: String,
}
