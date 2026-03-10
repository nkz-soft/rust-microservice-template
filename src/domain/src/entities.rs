use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity;
use crate::schema::to_do_items;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = to_do_items)]
pub struct ToDoItem {
    pub id: Uuid,
    pub title: Option<String>,
    pub note: Option<String>,
    pub version: i32,
}

impl ToDoItem {
    pub fn new(title: String, note: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Some(title),
            note: Some(note),
            version: 1,
        }
    }

    pub fn new_id(id: Uuid, title: String, note: String) -> Self {
        Self::new_versioned(id, title, note, 1)
    }

    pub fn new_versioned(id: Uuid, title: String, note: String, version: i32) -> Self {
        Self {
            id,
            title: Some(title),
            note: Some(note),
            version,
        }
    }
}

impl entity::Entity<ToDoItem> for ToDoItem {}
