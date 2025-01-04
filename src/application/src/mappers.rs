#![allow(dead_code)]
use domain::ToDoItem;
use tokio_postgres::Row;

pub struct ToDoItemMapper {}

impl ToDoItemMapper {
    pub fn from(row: Row) -> ToDoItem {
        ToDoItem::new_id(row.get("id"), row.get("title"), row.get("note"))
    }

    pub fn from_vec(rows: Vec<Row>) -> Vec<ToDoItem> {
        rows.iter()
            .map(|row| ToDoItem::new_id(row.get("id"), row.get("title"), row.get("note")))
            .collect::<Vec<ToDoItem>>()
    }
}
