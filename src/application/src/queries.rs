use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToDoItemSortField {
    Id,
    Title,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToDoItemSort {
    pub field: ToDoItemSortField,
    pub direction: SortDirection,
}

impl Default for ToDoItemSort {
    fn default() -> Self {
        Self {
            field: ToDoItemSortField::Id,
            direction: SortDirection::Asc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAllToDoItemsQuery {
    pub page: u32,
    pub page_size: u32,
    pub search: Option<String>,
    pub sort: ToDoItemSort,
}

impl Default for GetAllToDoItemsQuery {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            search: None,
            sort: ToDoItemSort::default(),
        }
    }
}

impl GetAllToDoItemsQuery {
    pub fn new(page: u32, page_size: u32, search: Option<String>, sort: ToDoItemSort) -> Self {
        Self {
            page,
            page_size,
            search,
            sort,
        }
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.page_size) as i64
    }

    pub fn limit(&self) -> i64 {
        self.page_size as i64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
    pub total_pages: u32,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, page: u32, page_size: u32, total_items: i64) -> Self {
        let total_pages = if total_items == 0 {
            0
        } else {
            ((total_items + page_size as i64 - 1) / page_size as i64) as u32
        };

        Self {
            items,
            page,
            page_size,
            total_items,
            total_pages,
        }
    }
}

pub struct GetToDoItemQuery {
    pub id: Option<Uuid>,
}

impl GetToDoItemQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        GetToDoItemQuery { id }
    }
}

pub struct CreateToDoItemQuery {
    pub title: String,
    pub note: String,
    pub status: String,
    pub due_at: Option<DateTime<Utc>>,
}

impl CreateToDoItemQuery {
    pub fn new(
        title: &String,
        note: &String,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
    ) -> Self {
        CreateToDoItemQuery {
            title: title.into(),
            note: note.into(),
            status: status.into(),
            due_at,
        }
    }
}

pub struct UpdateToDoItemQuery {
    pub id: Uuid,
    pub title: String,
    pub note: String,
    pub status: String,
    pub due_at: Option<DateTime<Utc>>,
    pub version: i32,
}

impl UpdateToDoItemQuery {
    pub fn new(
        id: Uuid,
        title: &String,
        note: &String,
        status: impl Into<String>,
        due_at: Option<DateTime<Utc>>,
        version: i32,
    ) -> Self {
        UpdateToDoItemQuery {
            id,
            title: title.into(),
            note: note.into(),
            status: status.into(),
            due_at,
            version,
        }
    }
}

pub struct DeleteToDoItemQuery {
    pub id: Uuid,
}

impl DeleteToDoItemQuery {
    pub fn new(id: Uuid) -> Self {
        DeleteToDoItemQuery { id }
    }
}
