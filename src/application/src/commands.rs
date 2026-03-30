use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateToDoItemCommand {
    pub title: String,
    pub note: String,
    pub status: String,
    pub due_at: Option<DateTime<Utc>>,
}

impl CreateToDoItemCommand {
    pub fn new(
        title: impl Into<String>,
        note: impl Into<String>,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            title: title.into(),
            note: note.into(),
            status: status.into(),
            due_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateToDoItemCommand {
    pub id: Uuid,
    pub title: String,
    pub note: String,
    pub status: String,
    pub due_at: Option<DateTime<Utc>>,
    pub version: i32,
}

impl UpdateToDoItemCommand {
    pub fn new(
        id: Uuid,
        title: impl Into<String>,
        note: impl Into<String>,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
        version: i32,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            note: note.into(),
            status: status.into(),
            due_at,
            version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteToDoItemCommand {
    pub id: Uuid,
    pub deleted_by: Option<Uuid>,
}

impl DeleteToDoItemCommand {
    pub fn new(id: Uuid, deleted_by: Option<Uuid>) -> Self {
        Self { id, deleted_by }
    }
}
