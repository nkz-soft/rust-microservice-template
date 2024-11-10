use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

//TODO We need to implement mapping from entity to response
#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ToDoItemResponse {
    /// The id of the to-do item
    pub id: Uuid,
    /// The title of the to-do item
    pub title: String,
    /// The note of the to-do item
    pub note: String,
}
