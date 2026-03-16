use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity;

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ToDoItem {
    pub id: Uuid,
    pub title: Option<String>,
    pub note: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
    pub version: i32,
}

impl ToDoItem {
    pub fn new(title: String, note: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            title: Some(title),
            note: Some(note),
            status: "pending".into(),
            created_at: now,
            updated_at: now,
            due_at: None,
            version: 1,
        }
    }

    pub fn new_with_lifecycle(
        title: String,
        note: String,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            title: Some(title),
            note: Some(note),
            status: status.into(),
            created_at: now,
            updated_at: now,
            due_at,
            version: 1,
        }
    }

    pub fn new_versioned(
        id: Uuid,
        title: String,
        note: String,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
        version: i32,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            title: Some(title),
            note: Some(note),
            status: status.into(),
            created_at: now,
            updated_at: now,
            due_at,
            version,
        }
    }
}

impl entity::Entity<ToDoItem> for ToDoItem {}
