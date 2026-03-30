use crate::commands::{CreateToDoItemCommand, DeleteToDoItemCommand, UpdateToDoItemCommand};
use crate::queries::{GetAllToDoItemsQuery, GetDeletedToDoItemForAuditQuery, GetToDoItemQuery};
use crate::repositories::{ToDoItemCommandRepository, ToDoItemQueryRepository};
use crate::ApplicationResult;
use domain::ToDoItem;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetToDoItemQueryHandler {
    repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
}

impl GetToDoItemQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
    ) -> GetToDoItemQueryHandler {
        GetToDoItemQueryHandler { repository }
    }

    pub async fn execute(&self, query: GetToDoItemQuery) -> ApplicationResult<ToDoItem> {
        self.repository.get_by_id(query.id).await
    }
}

pub struct GetAllToDoItemsQueryHandler {
    repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
}

impl GetAllToDoItemsQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
    ) -> GetAllToDoItemsQueryHandler {
        GetAllToDoItemsQueryHandler { repository }
    }

    pub async fn execute(
        &self,
        query: GetAllToDoItemsQuery,
    ) -> ApplicationResult<crate::PaginatedResult<ToDoItem>> {
        self.repository.get_all(query).await
    }
}

pub struct CreateToDoItemCommandHandler {
    repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
}

impl CreateToDoItemCommandHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
    ) -> CreateToDoItemCommandHandler {
        CreateToDoItemCommandHandler { repository }
    }

    pub async fn execute(&self, command: CreateToDoItemCommand) -> ApplicationResult<Uuid> {
        self.repository
            .create(ToDoItem::new_with_lifecycle(
                command.title,
                command.note,
                command.status,
                command.due_at,
            ))
            .await
    }
}

pub struct UpdateToDoItemCommandHandler {
    repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
}

impl UpdateToDoItemCommandHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
    ) -> UpdateToDoItemCommandHandler {
        UpdateToDoItemCommandHandler { repository }
    }

    pub async fn execute(&self, command: UpdateToDoItemCommand) -> ApplicationResult<Uuid> {
        self.repository
            .update(ToDoItem::new_versioned(
                command.id,
                command.title,
                command.note,
                command.status,
                command.due_at,
                command.version,
            ))
            .await
    }
}

pub struct DeleteToDoItemCommandHandler {
    repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
}

impl DeleteToDoItemCommandHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
    ) -> DeleteToDoItemCommandHandler {
        DeleteToDoItemCommandHandler { repository }
    }

    pub async fn execute(&self, command: DeleteToDoItemCommand) -> ApplicationResult<()> {
        self.repository.delete(command.id, command.deleted_by).await
    }
}

pub struct GetDeletedToDoItemForAuditQueryHandler {
    repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
}

impl GetDeletedToDoItemForAuditQueryHandler {
    pub fn new(
        repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ApplicationError, PaginatedResult};
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    struct QueryOnlyRepository {
        items: Arc<Mutex<Vec<ToDoItem>>>,
    }

    impl QueryOnlyRepository {
        fn with_item(item: ToDoItem) -> Self {
            Self {
                items: Arc::new(Mutex::new(vec![item])),
            }
        }
    }

    #[async_trait]
    impl ToDoItemQueryRepository for QueryOnlyRepository {
        async fn get_all(
            &self,
            query: GetAllToDoItemsQuery,
        ) -> ApplicationResult<PaginatedResult<ToDoItem>> {
            let items = self.items.lock().expect("items lock").clone();
            Ok(PaginatedResult::new(items, query.page, query.page_size, 1))
        }

        async fn get_by_id(&self, id: Uuid) -> ApplicationResult<ToDoItem> {
            self.items
                .lock()
                .expect("items lock")
                .iter()
                .find(|item| item.id == id)
                .cloned()
                .ok_or(ApplicationError::NotFound { id })
        }

        async fn get_deleted_by_id_for_audit(&self, id: Uuid) -> ApplicationResult<ToDoItem> {
            self.get_by_id(id).await
        }
    }

    struct CommandOnlyRepository {
        created: Arc<Mutex<Vec<ToDoItem>>>,
        deleted: Arc<Mutex<Vec<Uuid>>>,
    }

    impl CommandOnlyRepository {
        fn new() -> Self {
            Self {
                created: Arc::new(Mutex::new(Vec::new())),
                deleted: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl ToDoItemCommandRepository for CommandOnlyRepository {
        async fn create(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
            let id = entity.id;
            self.created.lock().expect("created lock").push(entity);
            Ok(id)
        }

        async fn update(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
            Ok(entity.id)
        }

        async fn delete(&self, id: Uuid, _deleted_by: Option<Uuid>) -> ApplicationResult<()> {
            self.deleted.lock().expect("deleted lock").push(id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn query_handlers_work_with_query_repository_only() {
        let item = ToDoItem::new_with_lifecycle(
            "title".to_string(),
            "note".to_string(),
            "pending",
            Some(Utc::now()),
        );
        let id = item.id;
        let repository = Arc::new(QueryOnlyRepository::with_item(item));

        let get_handler = GetToDoItemQueryHandler::new(repository.clone());
        let list_handler = GetAllToDoItemsQueryHandler::new(repository);

        let fetched = get_handler.execute(GetToDoItemQuery::new(id)).await;
        let listed = list_handler.execute(GetAllToDoItemsQuery::default()).await;

        assert!(fetched.is_ok());
        assert_eq!(listed.expect("list result").items.len(), 1);
    }

    #[tokio::test]
    async fn command_handlers_work_with_command_repository_only() {
        let repository = Arc::new(CommandOnlyRepository::new());
        let create_handler = CreateToDoItemCommandHandler::new(repository.clone());
        let delete_handler = DeleteToDoItemCommandHandler::new(repository.clone());

        let created_id = create_handler
            .execute(CreateToDoItemCommand::new("title", "note", "pending", None))
            .await
            .expect("create result");
        delete_handler
            .execute(DeleteToDoItemCommand::new(created_id, None))
            .await
            .expect("delete result");

        assert_eq!(repository.created.lock().expect("created lock").len(), 1);
        assert_eq!(
            repository.deleted.lock().expect("deleted lock").as_slice(),
            [created_id]
        );
    }
}
