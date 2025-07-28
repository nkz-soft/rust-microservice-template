use crate::queries::*;
use crate::repositories::*;
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

    pub async fn execute(&self, query: GetToDoItemQuery) -> anyhow::Result<ToDoItem> {
        self.repository.get_by_id(query.id.unwrap()).await
    }
}

pub struct GetAllToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl GetAllToDoItemQueryHandler {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> GetAllToDoItemQueryHandler {
        GetAllToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<ToDoItem>> {
        self.repository.get_all().await
    }
}

pub struct CreateToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl CreateToDoItemQueryHandler {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> CreateToDoItemQueryHandler {
        CreateToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: CreateToDoItemQuery) -> anyhow::Result<Uuid> {
        self.repository
            .save(ToDoItem::new(query.title, query.note))
            .await
    }
}

pub struct UpdateToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl UpdateToDoItemQueryHandler {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> UpdateToDoItemQueryHandler {
        UpdateToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: UpdateToDoItemQuery) -> anyhow::Result<Uuid> {
        self.repository
            .save(ToDoItem::new_id(query.id, query.title, query.note))
            .await
    }
}

pub struct DeleteToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl DeleteToDoItemQueryHandler {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> DeleteToDoItemQueryHandler {
        DeleteToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: DeleteToDoItemQuery) -> anyhow::Result<()> {
        self.repository.delete(query.id).await
    }
}
