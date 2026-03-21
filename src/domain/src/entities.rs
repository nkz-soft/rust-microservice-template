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
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
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
            deleted_at: None,
            deleted_by: None,
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
            deleted_at: None,
            deleted_by: None,
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
            deleted_at: None,
            deleted_by: None,
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_active(&self) -> bool {
        !self.is_deleted()
    }

    pub fn mark_deleted_once(&mut self, deleted_by: Option<Uuid>) {
        if self.deleted_at.is_none() {
            self.deleted_at = Some(Utc::now());
            self.deleted_by = deleted_by;
        }
    }
}

impl entity::Entity<ToDoItem> for ToDoItem {}

#[cfg(test)]
mod tests {
    use super::ToDoItem;
    use uuid::Uuid;

    #[test]
    fn mark_deleted_once_sets_metadata_only_on_first_call() {
        let mut item = ToDoItem::new("title".into(), "note".into());
        let actor = Some(Uuid::new_v4());

        item.mark_deleted_once(actor);
        let first_deleted_at = item.deleted_at;
        let first_deleted_by = item.deleted_by;
        item.mark_deleted_once(Some(Uuid::new_v4()));

        assert!(item.is_deleted());
        assert!(!item.is_active());
        assert_eq!(item.deleted_at, first_deleted_at);
        assert_eq!(item.deleted_by, first_deleted_by);
    }

    #[test]
    fn mark_deleted_once_supports_missing_actor() {
        let mut item = ToDoItem::new("title".into(), "note".into());

        item.mark_deleted_once(None);

        assert!(item.deleted_at.is_some());
        assert_eq!(item.deleted_by, None);
    }
}
