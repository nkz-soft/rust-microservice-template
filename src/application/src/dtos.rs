use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct ToDoItemDto {
    pub id: Uuid,
    pub title: String,
    pub note: String,
}

impl ToDoItemDto {
    pub fn get() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Test title".into(),
            note: "Test note".into(),
        }
    }
}
