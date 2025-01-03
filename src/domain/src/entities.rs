use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity;
use crate::schema::to_do_items;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, PartialEq, Debug)]
#[diesel(table_name = to_do_items)]
pub struct ToDoItem {
    pub id: Uuid,
    pub title: Option<String>,
    pub note: Option<String>,
}

impl ToDoItem {
    pub fn new(title: String, note: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Some(title),
            note: Some(note),
        }
    }

    pub fn new_id(id: Uuid, title: String, note: String) -> Self {
        Self {
            id,
            title: Some(title),
            note: Some(note),
        }
    }
}

impl entity::Entity<ToDoItem> for ToDoItem {}
