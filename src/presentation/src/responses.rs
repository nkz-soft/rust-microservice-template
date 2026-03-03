use domain::ToDoItem;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ToDoItemResponse {
    /// The id of the to-do item
    pub id: Uuid,
    /// The title of the to-do item
    pub title: Option<String>,
    /// The note of the to-do item
    pub note: Option<String>,
}

impl From<ToDoItem> for ToDoItemResponse {
    fn from(item: ToDoItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            note: item.note,
        }
    }
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ProblemDetailsResponse {
    /// HTTP status code.
    pub status: u16,
    /// Human-readable explanation specific to this occurrence of the problem.
    pub detail: String,
}
