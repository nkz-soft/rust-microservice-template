use application::PaginatedResult;
use domain::ToDoItem;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ToDoItemResponse {
    /// The id of the to-do item
    pub id: Uuid,
    /// The title of the to-do item
    pub title: Option<String>,
    /// The note of the to-do item
    pub note: Option<String>,
}

impl From<ToDoItem> for ToDoItemResponse {
    fn from(item: ToDoItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            note: item.note,
        }
    }
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct PaginationMetaResponse {
    /// One-based page number.
    pub page: u32,
    /// Number of items requested per page.
    pub page_size: u32,
    /// Total number of matching items.
    pub total_items: i64,
    /// Total number of pages for the current filter.
    pub total_pages: u32,
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ToDoItemsPageResponse {
    /// Current page of to-do items.
    pub items: Vec<ToDoItemResponse>,
    /// Pagination metadata for the current result set.
    pub meta: PaginationMetaResponse,
}

impl From<PaginatedResult<ToDoItem>> for ToDoItemsPageResponse {
    fn from(result: PaginatedResult<ToDoItem>) -> Self {
        Self {
            items: result
                .items
                .into_iter()
                .map(ToDoItemResponse::from)
                .collect(),
            meta: PaginationMetaResponse {
                page: result.page,
                page_size: result.page_size,
                total_items: result.total_items,
                total_pages: result.total_pages,
            },
        }
    }
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ProblemDetailsResponse {
    /// HTTP status code.
    pub status: u16,
    /// Human-readable explanation specific to this occurrence of the problem.
    pub detail: String,
}
