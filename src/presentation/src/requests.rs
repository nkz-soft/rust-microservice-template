use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct CreateToDoItemRequest {
    pub title: String,
    pub note: String
}

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct UpdateToDoItemRequest {
    pub id: Uuid,
    pub title: String,
    pub note: String
}
