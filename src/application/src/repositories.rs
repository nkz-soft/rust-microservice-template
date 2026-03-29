use async_trait::async_trait;
use domain::ToDoItem;
use uuid::Uuid;

use crate::{ApplicationResult, GetAllToDoItemsQuery, PaginatedResult};

#[async_trait]
pub trait ToDoItemRepository: Send + Sync {
    async fn get_all(
        &self,
        query: GetAllToDoItemsQuery,
    ) -> ApplicationResult<PaginatedResult<ToDoItem>>;
    async fn get_by_id(&self, id: Uuid) -> ApplicationResult<ToDoItem>;
    async fn get_deleted_by_id_for_audit(&self, id: Uuid) -> ApplicationResult<ToDoItem>;
    async fn create(&self, entity: ToDoItem) -> ApplicationResult<Uuid>;
    async fn update(&self, entity: ToDoItem) -> ApplicationResult<Uuid>;
    async fn delete(&self, id: Uuid, deleted_by: Option<Uuid>) -> ApplicationResult<()>;
}
