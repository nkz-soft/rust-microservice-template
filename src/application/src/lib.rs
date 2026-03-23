mod dtos;
mod handlers;
mod mappers;
mod queries;
mod repositories;
mod services;
mod settings;

pub use crate::settings::{AuthSettings, AuthUser, ServiceApiKey, Settings};

pub use crate::repositories::ToDoItemRepository;

pub use crate::dtos::{
    AuthenticatedPrincipal, Permission, PrincipalType, ToDoItemDto, TokenClaims, TokenResponse,
};

pub use crate::queries::{
    audit_read_policy, todo_read_policy, todo_write_policy, CreateToDoItemQuery,
    DeleteToDoItemQuery, GetAllToDoItemsQuery, GetDeletedToDoItemForAuditQuery, GetToDoItemQuery,
    LoginQuery, PaginatedResult, ProtectedEndpointPolicy, SortDirection, ToDoItemSort,
    ToDoItemSortField, UpdateToDoItemQuery,
};

pub use crate::handlers::{
    CreateToDoItemQueryHandler, DeleteToDoItemQueryHandler, GetAllToDoItemQueryHandler,
    GetDeletedToDoItemForAuditQueryHandler, GetToDoItemQueryHandler, LoginQueryHandler,
    UpdateToDoItemQueryHandler,
};

pub use crate::services::{AuthError, AuthService, ToDoItemService, ToDoItemServiceBoxed};
