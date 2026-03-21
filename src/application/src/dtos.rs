use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct ToDoItemDto {
    pub id: Uuid,
    pub title: String,
    pub note: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl ToDoItemDto {
    pub fn get() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Test title".into(),
            note: "Test note".into(),
            status: "pending".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_at: None,
            deleted_at: None,
            deleted_by: None,
        }
    }
}
