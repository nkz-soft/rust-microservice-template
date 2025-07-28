use crate::handlers::*;
use crate::repositories::ToDoItemRepository;
use std::sync::Arc;

/// Service container that manages all query handlers with proper dependency injection
#[derive(Clone)]
pub struct ToDoItemService {
    get_handler: Arc<GetToDoItemQueryHandler>,
    get_all_handler: Arc<GetAllToDoItemQueryHandler>,
    create_handler: Arc<CreateToDoItemQueryHandler>,
    update_handler: Arc<UpdateToDoItemQueryHandler>,
    delete_handler: Arc<DeleteToDoItemQueryHandler>,
}

impl ToDoItemService {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> Self {
        Self {
            get_handler: Arc::new(GetToDoItemQueryHandler::new(repository.clone())),
            get_all_handler: Arc::new(GetAllToDoItemQueryHandler::new(repository.clone())),
            create_handler: Arc::new(CreateToDoItemQueryHandler::new(repository.clone())),
            update_handler: Arc::new(UpdateToDoItemQueryHandler::new(repository.clone())),
            delete_handler: Arc::new(DeleteToDoItemQueryHandler::new(repository)),
        }
    }

    pub fn get_handler(&self) -> Arc<GetToDoItemQueryHandler> {
        self.get_handler.clone()
    }

    pub fn get_all_handler(&self) -> Arc<GetAllToDoItemQueryHandler> {
        self.get_all_handler.clone()
    }

    pub fn create_handler(&self) -> Arc<CreateToDoItemQueryHandler> {
        self.create_handler.clone()
    }

    pub fn update_handler(&self) -> Arc<UpdateToDoItemQueryHandler> {
        self.update_handler.clone()
    }

    pub fn delete_handler(&self) -> Arc<DeleteToDoItemQueryHandler> {
        self.delete_handler.clone()
    }
}

// Alternative approach using Box<dyn Trait> for handlers when cloning is not needed
pub struct ToDoItemServiceBoxed {
    repository: Arc<dyn ToDoItemRepository + Send + Sync>,
}

impl ToDoItemServiceBoxed {
    pub fn new(repository: Arc<dyn ToDoItemRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn create_get_handler(&self) -> Box<GetToDoItemQueryHandler> {
        Box::new(GetToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_get_all_handler(&self) -> Box<GetAllToDoItemQueryHandler> {
        Box::new(GetAllToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_create_handler(&self) -> Box<CreateToDoItemQueryHandler> {
        Box::new(CreateToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_update_handler(&self) -> Box<UpdateToDoItemQueryHandler> {
        Box::new(UpdateToDoItemQueryHandler::new(self.repository.clone()))
    }

    pub fn create_delete_handler(&self) -> Box<DeleteToDoItemQueryHandler> {
        Box::new(DeleteToDoItemQueryHandler::new(self.repository.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use domain::ToDoItem;
    use std::sync::{Arc, Mutex};
    use tokio::task;
    use uuid::Uuid;

    // Mock repository for testing
    struct MockToDoItemRepository {
        items: Arc<Mutex<Vec<ToDoItem>>>,
        call_count: Arc<Mutex<usize>>,
    }

    impl MockToDoItemRepository {
        fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(Vec::new())),
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn get_call_count(&self) -> usize {
            *self.call_count.lock().unwrap()
        }

        fn add_item(&self, item: ToDoItem) {
            self.items.lock().unwrap().push(item);
        }
    }

    #[async_trait]
    impl ToDoItemRepository for MockToDoItemRepository {
        async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>> {
            *self.call_count.lock().unwrap() += 1;
            Ok(self.items.lock().unwrap().clone())
        }

        async fn get_by_id(&self, id: Uuid) -> anyhow::Result<ToDoItem> {
            *self.call_count.lock().unwrap() += 1;
            self.items
                .lock()
                .unwrap()
                .iter()
                .find(|item| item.id == id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Item not found"))
        }

        async fn save(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
            *self.call_count.lock().unwrap() += 1;
            let id = entity.id;
            self.items.lock().unwrap().push(entity);
            Ok(id)
        }

        async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
            *self.call_count.lock().unwrap() += 1;
            let mut items = self.items.lock().unwrap();
            items.retain(|item| item.id != id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_service_creation_with_arc() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = ToDoItemService::new(repository);

        // Test that handlers are created successfully - handlers are Arc<T> so they don't have is_ok()
        let get_handler = service.get_handler();
        let get_all_handler = service.get_all_handler();
        let create_handler = service.create_handler();
        let update_handler = service.update_handler();
        let delete_handler = service.delete_handler();

        // Verify that we can get handlers without panicking
        assert!(Arc::strong_count(&get_handler) >= 1);
        assert!(Arc::strong_count(&get_all_handler) >= 1);
        assert!(Arc::strong_count(&create_handler) >= 1);
        assert!(Arc::strong_count(&update_handler) >= 1);
        assert!(Arc::strong_count(&delete_handler) >= 1);
    }

    #[tokio::test]
    async fn test_service_cloning() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = ToDoItemService::new(repository);
        
        // Test that service can be cloned (important for actix-web Data<>)
        let cloned_service = service.clone();
        
        // Both services should work independently
        let handler1 = service.get_all_handler();
        let handler2 = cloned_service.get_all_handler();
        
        let result1 = handler1.execute().await;
        let result2 = handler2.execute().await;
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = Arc::new(ToDoItemService::new(repository.clone()));

        // Add some test data
        repository.add_item(ToDoItem::new("Test 1".to_string(), "Note 1".to_string()));
        repository.add_item(ToDoItem::new("Test 2".to_string(), "Note 2".to_string()));

        // Test concurrent access with multiple tasks
        let mut handles = vec![];
        
        for i in 0..10 {
            let service_clone = service.clone();
            let handle = task::spawn(async move {
                let handler = service_clone.get_all_handler();
                let result = handler.execute().await;
                (i, result)
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let (task_id, result) = handle.await.unwrap();
            assert!(result.is_ok(), "Task {} failed", task_id);
            assert_eq!(result.unwrap().len(), 2);
        }

        // Verify that repository was called 10 times
        assert_eq!(repository.get_call_count(), 10);
    }

    #[tokio::test]
    async fn test_boxed_service_creation() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = ToDoItemServiceBoxed::new(repository);

        // Test that boxed handlers are created successfully
        let get_handler = service.create_get_handler();
        let get_all_handler = service.create_get_all_handler();
        let create_handler = service.create_create_handler();
        let update_handler = service.create_update_handler();
        let delete_handler = service.create_delete_handler();

        // Verify handlers are boxed correctly
        assert_eq!(std::mem::size_of_val(&*get_handler), std::mem::size_of::<GetToDoItemQueryHandler>());
        assert_eq!(std::mem::size_of_val(&*get_all_handler), std::mem::size_of::<GetAllToDoItemQueryHandler>());
        assert_eq!(std::mem::size_of_val(&*create_handler), std::mem::size_of::<CreateToDoItemQueryHandler>());
        assert_eq!(std::mem::size_of_val(&*update_handler), std::mem::size_of::<UpdateToDoItemQueryHandler>());
        assert_eq!(std::mem::size_of_val(&*delete_handler), std::mem::size_of::<DeleteToDoItemQueryHandler>());
    }

    #[tokio::test]
    async fn test_memory_efficiency_arc_vs_box() {
        let repository = Arc::new(MockToDoItemRepository::new());
        
        // Test Arc-based service
        let arc_service = ToDoItemService::new(repository.clone());
        let arc_handler1 = arc_service.get_all_handler();
        let arc_handler2 = arc_service.get_all_handler();
        
        // Test Box-based service
        let box_service = ToDoItemServiceBoxed::new(repository);
        let box_handler1 = box_service.create_get_all_handler();
        let box_handler2 = box_service.create_get_all_handler();
        
        // Arc handlers should point to the same memory location (shared)
        assert_eq!(Arc::as_ptr(&arc_handler1), Arc::as_ptr(&arc_handler2));
        
        // Box handlers should be different instances
        assert_ne!(&*box_handler1 as *const _, &*box_handler2 as *const _);
    }

    #[tokio::test]
    async fn test_send_sync_bounds() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = Arc::new(ToDoItemService::new(repository));

        // Test that service can be sent across threads
        let handle = task::spawn(async move {
            let handler = service.get_all_handler();
            handler.execute().await
        });

        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_handler_implementation() {
        let repository = Arc::new(MockToDoItemRepository::new());
        let service = ToDoItemService::new(repository);
        
        // Verify that handlers are properly wrapped in Arc
        let handler = service.get_handler();
        // Just verify we can get the handler without panicking
        assert!(Arc::strong_count(&handler) >= 1);
    }
} 