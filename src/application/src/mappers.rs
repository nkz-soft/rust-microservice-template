#![allow(dead_code)]
use chrono::{DateTime, Utc};
use domain::ToDoItem;
use std::time::SystemTime;
use tokio_postgres::Row;

pub struct ToDoItemMapper {}

impl ToDoItemMapper {
    pub fn from(row: Row) -> ToDoItem {
        ToDoItem {
            id: row.get("id"),
            title: row.get("title"),
            note: row.get("note"),
            status: row.get("status"),
            created_at: DateTime::<Utc>::from(row.get::<_, SystemTime>("created_at")),
            updated_at: DateTime::<Utc>::from(row.get::<_, SystemTime>("updated_at")),
            due_at: row
                .get::<_, Option<SystemTime>>("due_at")
                .map(DateTime::<Utc>::from),
            version: row.get("version"),
        }
    }

    pub fn from_vec(rows: Vec<Row>) -> Vec<ToDoItem> {
        rows.into_iter().map(Self::from).collect::<Vec<ToDoItem>>()
    }
}
