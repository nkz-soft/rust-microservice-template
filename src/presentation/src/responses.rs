use application::PaginatedResult;
use application::TokenResponse;
use chrono::{DateTime, Utc};
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
    /// Lifecycle status.
    pub status: String,
    /// Creation timestamp in UTC.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp in UTC.
    pub updated_at: DateTime<Utc>,
    /// Optional due timestamp in UTC.
    pub due_at: Option<DateTime<Utc>>,
}

impl From<ToDoItem> for ToDoItemResponse {
    fn from(item: ToDoItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            note: item.note,
            status: item.status,
            created_at: item.created_at,
            updated_at: item.updated_at,
            due_at: item.due_at,
        }
    }
}

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuditToDoItemResponse {
    /// The id of the to-do item
    pub id: Uuid,
    /// The title of the to-do item
    pub title: Option<String>,
    /// The note of the to-do item
    pub note: Option<String>,
    /// Lifecycle status.
    pub status: String,
    /// Creation timestamp in UTC.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp in UTC.
    pub updated_at: DateTime<Utc>,
    /// Optional due timestamp in UTC.
    pub due_at: Option<DateTime<Utc>>,
    /// Deletion timestamp in UTC.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Optional actor that performed deletion.
    pub deleted_by: Option<Uuid>,
}

impl From<ToDoItem> for AuditToDoItemResponse {
    fn from(item: ToDoItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            note: item.note,
            status: item.status,
            created_at: item.created_at,
            updated_at: item.updated_at,
            due_at: item.due_at,
            deleted_at: item.deleted_at,
            deleted_by: item.deleted_by,
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

#[readonly::make]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct TokenResponseBody {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub permissions: Vec<String>,
}

impl From<TokenResponse> for TokenResponseBody {
    fn from(response: TokenResponse) -> Self {
        Self {
            access_token: response.access_token,
            token_type: response.token_type,
            expires_in: response.expires_in,
            permissions: response.permissions,
        }
    }
}
