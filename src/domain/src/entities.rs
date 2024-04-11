use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity;

#[derive(Serialize, Deserialize)]
pub struct ToDoItem {
    pub id: Uuid,
    pub title: String,
    pub note: String,
}

impl ToDoItem {
    pub fn new(title: String, note: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            note,
        }
    }

    pub fn new_id(id: Uuid, title: String, note: String) -> Self {
        Self { id, title, note }
    }
}

impl entity::Entity<ToDoItem> for ToDoItem {}
