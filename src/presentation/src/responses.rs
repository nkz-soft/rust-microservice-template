use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

//TODO We need to implement mapping from entity to response
#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ToDoItemResponse {
    pub id: Uuid,
    pub title: String,
    pub note: String,
}
