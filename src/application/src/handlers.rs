use crate::queries::*;
use crate::repositories::*;
use crate::ApplicationResult;
use domain::ToDoItem;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl GetToDoItemQueryHandler {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> GetToDoItemQueryHandler {
        GetToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: GetToDoItemQuery) -> ApplicationResult<ToDoItem> {
        self.repository.get_by_id(query.id.unwrap()).await
    }
}

pub struct GetAllToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl GetAllToDoItemQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemRepository + Send + Sync>,
    ) -> GetAllToDoItemQueryHandler {
        GetAllToDoItemQueryHandler { repository }
    }

    pub async fn execute(
        &self,
        query: GetAllToDoItemsQuery,
    ) -> ApplicationResult<PaginatedResult<ToDoItem>> {
        self.repository.get_all(query).await
    }
}

pub struct CreateToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl CreateToDoItemQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemRepository + Send + Sync>,
    ) -> CreateToDoItemQueryHandler {
        CreateToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: CreateToDoItemQuery) -> ApplicationResult<Uuid> {
        self.repository
            .create(ToDoItem::new_with_lifecycle(
                query.title,
                query.note,
                query.status,
                query.due_at,
            ))
            .await
    }
}

pub struct UpdateToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl UpdateToDoItemQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemRepository + Send + Sync>,
    ) -> UpdateToDoItemQueryHandler {
        UpdateToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: UpdateToDoItemQuery) -> ApplicationResult<Uuid> {
        self.repository
            .update(ToDoItem::new_versioned(
                query.id,
                query.title,
                query.note,
                query.status,
                query.due_at,
                query.version,
            ))
            .await
    }
}

pub struct DeleteToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl DeleteToDoItemQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemRepository + Send + Sync>,
    ) -> DeleteToDoItemQueryHandler {
        DeleteToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: DeleteToDoItemQuery) -> ApplicationResult<()> {
        self.repository.delete(query.id, query.deleted_by).await
    }
}

pub struct GetDeletedToDoItemForAuditQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl GetDeletedToDoItemForAuditQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemRepository + Send + Sync>,
    ) -> GetDeletedToDoItemForAuditQueryHandler {
        GetDeletedToDoItemForAuditQueryHandler { repository }
    }

    pub async fn execute(
        &self,
        query: GetDeletedToDoItemForAuditQuery,
    ) -> ApplicationResult<ToDoItem> {
        self.repository.get_deleted_by_id_for_audit(query.id).await
    }
}
