use crate::handlers::{
    CreateToDoItemCommandHandler, DeleteToDoItemCommandHandler, GetAllToDoItemsQueryHandler,
    GetDeletedToDoItemForAuditQueryHandler, GetToDoItemQueryHandler, UpdateToDoItemCommandHandler,
};
use crate::repositories::{ToDoItemCommandRepository, ToDoItemQueryRepository};
use std::sync::Arc;

/// Service container that manages command and query handlers with explicit CQRS boundaries.
#[derive(Clone)]
pub struct ToDoItemService {
    get_query_handler: Arc<GetToDoItemQueryHandler>,
    get_all_query_handler: Arc<GetAllToDoItemsQueryHandler>,
    create_command_handler: Arc<CreateToDoItemCommandHandler>,
    update_command_handler: Arc<UpdateToDoItemCommandHandler>,
    delete_command_handler: Arc<DeleteToDoItemCommandHandler>,
    get_deleted_for_audit_query_handler: Arc<GetDeletedToDoItemForAuditQueryHandler>,
}

impl ToDoItemService {
    pub fn new(
        query_repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
        command_repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
    ) -> Self {
        Self {
            get_query_handler: Arc::new(GetToDoItemQueryHandler::new(query_repository.clone())),
            get_all_query_handler: Arc::new(GetAllToDoItemsQueryHandler::new(
                query_repository.clone(),
            )),
            create_command_handler: Arc::new(CreateToDoItemCommandHandler::new(
                command_repository.clone(),
            )),
            update_command_handler: Arc::new(UpdateToDoItemCommandHandler::new(
                command_repository.clone(),
            )),
            delete_command_handler: Arc::new(DeleteToDoItemCommandHandler::new(command_repository)),
            get_deleted_for_audit_query_handler: Arc::new(
                GetDeletedToDoItemForAuditQueryHandler::new(query_repository),
            ),
        }
    }

    pub fn get_query_handler(&self) -> Arc<GetToDoItemQueryHandler> {
        self.get_query_handler.clone()
    }

    pub fn get_all_query_handler(&self) -> Arc<GetAllToDoItemsQueryHandler> {
        self.get_all_query_handler.clone()
    }

    pub fn create_command_handler(&self) -> Arc<CreateToDoItemCommandHandler> {
        self.create_command_handler.clone()
    }

    pub fn update_command_handler(&self) -> Arc<UpdateToDoItemCommandHandler> {
        self.update_command_handler.clone()
    }

    pub fn delete_command_handler(&self) -> Arc<DeleteToDoItemCommandHandler> {
        self.delete_command_handler.clone()
    }

    pub fn get_deleted_for_audit_query_handler(
        &self,
    ) -> Arc<GetDeletedToDoItemForAuditQueryHandler> {
        self.get_deleted_for_audit_query_handler.clone()
    }
}

pub struct ToDoItemServiceBoxed {
    query_repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
    command_repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
}

impl ToDoItemServiceBoxed {
    pub fn new(
        query_repository: Arc<dyn ToDoItemQueryRepository + Send + Sync>,
        command_repository: Arc<dyn ToDoItemCommandRepository + Send + Sync>,
    ) -> Self {
        Self {
            query_repository,
            command_repository,
        }
    }

    pub fn create_get_query_handler(&self) -> Box<GetToDoItemQueryHandler> {
        Box::new(GetToDoItemQueryHandler::new(self.query_repository.clone()))
    }

    pub fn create_get_all_query_handler(&self) -> Box<GetAllToDoItemsQueryHandler> {
        Box::new(GetAllToDoItemsQueryHandler::new(
            self.query_repository.clone(),
        ))
    }

    pub fn create_create_command_handler(&self) -> Box<CreateToDoItemCommandHandler> {
        Box::new(CreateToDoItemCommandHandler::new(
            self.command_repository.clone(),
        ))
    }

    pub fn create_update_command_handler(&self) -> Box<UpdateToDoItemCommandHandler> {
        Box::new(UpdateToDoItemCommandHandler::new(
            self.command_repository.clone(),
        ))
    }

    pub fn create_delete_command_handler(&self) -> Box<DeleteToDoItemCommandHandler> {
        Box::new(DeleteToDoItemCommandHandler::new(
            self.command_repository.clone(),
        ))
    }

    pub fn create_get_deleted_for_audit_query_handler(
        &self,
    ) -> Box<GetDeletedToDoItemForAuditQueryHandler> {
        Box::new(GetDeletedToDoItemForAuditQueryHandler::new(
            self.query_repository.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ApplicationError, ApplicationResult, CreateToDoItemCommand, GetAllToDoItemsQuery,
        GetToDoItemQuery, PaginatedResult, UpdateToDoItemCommand,
    };
    use async_trait::async_trait;
    use domain::ToDoItem;
    use std::sync::{Arc, Mutex};
    use tokio::task;
    use uuid::Uuid;

    struct SharedRepositoryState {
        items: Arc<Mutex<Vec<ToDoItem>>>,
        query_call_count: Arc<Mutex<usize>>,
        command_call_count: Arc<Mutex<usize>>,
    }

    impl SharedRepositoryState {
        fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(Vec::new())),
                query_call_count: Arc::new(Mutex::new(0)),
                command_call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn add_item(&self, item: ToDoItem) {
            self.items.lock().expect("items lock").push(item);
        }
    }

    #[async_trait]
    impl ToDoItemQueryRepository for SharedRepositoryState {
        async fn get_all(
            &self,
            query: GetAllToDoItemsQuery,
        ) -> ApplicationResult<PaginatedResult<ToDoItem>> {
            *self.query_call_count.lock().expect("query count lock") += 1;
            let items = self.items.lock().expect("items lock").clone();
            let total_items = items.len() as i64;
            let paged_items = items
                .into_iter()
                .skip(query.offset() as usize)
                .take(query.limit() as usize)
                .collect();

            Ok(PaginatedResult::new(
                paged_items,
                query.page,
                query.page_size,
                total_items,
            ))
        }

        async fn get_by_id(&self, id: Uuid) -> ApplicationResult<ToDoItem> {
            *self.query_call_count.lock().expect("query count lock") += 1;
            self.items
                .lock()
                .expect("items lock")
                .iter()
                .find(|item| item.id == id)
                .cloned()
                .ok_or(ApplicationError::NotFound { id })
        }

        async fn get_deleted_by_id_for_audit(&self, id: Uuid) -> ApplicationResult<ToDoItem> {
            *self.query_call_count.lock().expect("query count lock") += 1;
            self.items
                .lock()
                .expect("items lock")
                .iter()
                .find(|item| item.id == id && item.deleted_at.is_some())
                .cloned()
                .ok_or(ApplicationError::NotFound { id })
        }
    }

    #[async_trait]
    impl ToDoItemCommandRepository for SharedRepositoryState {
        async fn create(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
            *self.command_call_count.lock().expect("command count lock") += 1;
            let id = entity.id;
            self.items.lock().expect("items lock").push(entity);
            Ok(id)
        }

        async fn update(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
            *self.command_call_count.lock().expect("command count lock") += 1;
            let id = entity.id;
            let mut items = self.items.lock().expect("items lock");
            let existing = items
                .iter_mut()
                .find(|item| item.id == id)
                .ok_or(ApplicationError::NotFound { id })?;

            if existing.version != entity.version {
                return Err(ApplicationError::Conflict {
                    id,
                    expected_version: entity.version,
                    actual_version: existing.version,
                });
            }

            existing.title = entity.title;
            existing.note = entity.note;
            existing.status = entity.status;
            existing.due_at = entity.due_at;
            existing.version += 1;

            Ok(id)
        }

        async fn delete(&self, id: Uuid, _deleted_by: Option<Uuid>) -> ApplicationResult<()> {
            *self.command_call_count.lock().expect("command count lock") += 1;
            self.items
                .lock()
                .expect("items lock")
                .retain(|item| item.id != id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn service_clones_share_query_handlers() {
        let repository = Arc::new(SharedRepositoryState::new());
        let service = ToDoItemService::new(repository.clone(), repository);

        let cloned = service.clone();

        assert_eq!(
            Arc::as_ptr(&service.get_all_query_handler()),
            Arc::as_ptr(&cloned.get_all_query_handler())
        );
    }

    #[tokio::test]
    async fn query_handlers_only_increment_query_side_state() {
        let repository = Arc::new(SharedRepositoryState::new());
        let item = ToDoItem::new("title".to_string(), "note".to_string());
        let id = item.id;
        repository.add_item(item);
        let service = ToDoItemService::new(repository.clone(), repository.clone());

        let _ = service
            .get_query_handler()
            .execute(GetToDoItemQuery::new(id))
            .await
            .expect("get result");
        let _ = service
            .get_all_query_handler()
            .execute(GetAllToDoItemsQuery::default())
            .await
            .expect("list result");

        assert_eq!(
            *repository
                .query_call_count
                .lock()
                .expect("query count lock"),
            2
        );
        assert_eq!(
            *repository
                .command_call_count
                .lock()
                .expect("command count lock"),
            0
        );
    }

    #[tokio::test]
    async fn command_handlers_only_increment_command_side_state() {
        let repository = Arc::new(SharedRepositoryState::new());
        let service = ToDoItemService::new(repository.clone(), repository.clone());

        let created_id = service
            .create_command_handler()
            .execute(CreateToDoItemCommand::new("Title", "Note", "pending", None))
            .await
            .expect("create result");
        service
            .delete_command_handler()
            .execute(crate::DeleteToDoItemCommand::new(created_id, None))
            .await
            .expect("delete result");

        assert_eq!(
            *repository
                .query_call_count
                .lock()
                .expect("query count lock"),
            0
        );
        assert_eq!(
            *repository
                .command_call_count
                .lock()
                .expect("command count lock"),
            2
        );
    }

    #[tokio::test]
    async fn update_command_handler_propagates_typed_conflict_errors() {
        let repository = Arc::new(SharedRepositoryState::new());
        let existing = ToDoItem::new("Title".to_string(), "Note".to_string());
        let id = existing.id;
        repository.add_item(existing);
        let service = ToDoItemService::new(repository.clone(), repository);

        let result = service
            .update_command_handler()
            .execute(UpdateToDoItemCommand::new(
                id,
                "Updated",
                "Updated note",
                "pending",
                None,
                99,
            ))
            .await;

        assert!(matches!(
            result,
            Err(ApplicationError::Conflict {
                expected_version: 99,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn service_is_send_sync_for_query_execution() {
        let repository = Arc::new(SharedRepositoryState::new());
        let service = Arc::new(ToDoItemService::new(repository.clone(), repository));

        let handle = task::spawn(async move {
            let handler = service.get_all_query_handler();
            handler.execute(GetAllToDoItemsQuery::default()).await
        });

        assert!(handle.await.expect("task join").is_ok());
    }
}
