use uuid::Uuid;
use async_trait::async_trait;
use domain::entities::ToDoItem;

#[async_trait]
pub trait ToDoItemRepository {
    async fn get_all(&self) -> Result<Vec<ToDoItem>, String>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ToDoItem>, String>;
    async fn save(&self, entity: ToDoItem) -> Result<Uuid, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
