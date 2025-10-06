use async_trait::async_trait;
use domain::ToDoItem;
use uuid::Uuid;

#[async_trait]
pub trait ToDoItemRepository: Send + Sync {
    async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<ToDoItem>;
    async fn save(&self, entity: ToDoItem) -> anyhow::Result<Uuid>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
