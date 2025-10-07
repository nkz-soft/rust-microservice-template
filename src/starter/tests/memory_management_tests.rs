#[cfg(test)]
mod tests {
    use application::{ToDoItemRepository, ToDoItemService};
    use domain::ToDoItem;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    // Mock repository for testing memory patterns
    struct TestToDoItemRepository {
        items: Arc<Mutex<Vec<ToDoItem>>>,
        operation_count: Arc<Mutex<usize>>,
    }

    impl TestToDoItemRepository {
        fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(Vec::new())),
                operation_count: Arc::new(Mutex::new(0)),
            }
        }

        fn get_operation_count(&self) -> usize {
            *self.operation_count.lock().unwrap()
        }

        fn reset_count(&self) {
            *self.operation_count.lock().unwrap() = 0;
        }
    }

    #[async_trait::async_trait]
    impl ToDoItemRepository for TestToDoItemRepository {
        async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>> {
            *self.operation_count.lock().unwrap() += 1;
            sleep(Duration::from_millis(10)).await; // Simulate some work
            Ok(self.items.lock().unwrap().clone())
        }

        async fn get_by_id(&self, id: Uuid) -> anyhow::Result<ToDoItem> {
            *self.operation_count.lock().unwrap() += 1;
            sleep(Duration::from_millis(10)).await; // Simulate some work
            self.items
                .lock()
                .unwrap()
                .iter()
                .find(|item| item.id == id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Item not found"))
        }

        async fn save(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
            *self.operation_count.lock().unwrap() += 1;
            sleep(Duration::from_millis(10)).await; // Simulate some work
            let id = entity.id;

            let mut items = self.items.lock().unwrap();
            // Update existing or add new
            if let Some(pos) = items.iter().position(|item| item.id == id) {
                items[pos] = entity;
            } else {
                items.push(entity);
            }
            Ok(id)
        }

        async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
            *self.operation_count.lock().unwrap() += 1;
            sleep(Duration::from_millis(10)).await; // Simulate some work
            let mut items = self.items.lock().unwrap();
            items.retain(|item| item.id != id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_arc_memory_sharing() {
        let repository = Arc::new(TestToDoItemRepository::new());
        let service = ToDoItemService::new(repository.clone());

        // Create multiple handlers from the same service
        let handler1 = service.get_all_handler();
        let handler2 = service.get_all_handler();
        let handler3 = service.get_all_handler();

        // All handlers should point to the same Arc instance
        assert_eq!(Arc::as_ptr(&handler1), Arc::as_ptr(&handler2));
        assert_eq!(Arc::as_ptr(&handler2), Arc::as_ptr(&handler3));

        // Test that they can be used concurrently
        let (result1, result2, result3) =
            tokio::join!(handler1.execute(), handler2.execute(), handler3.execute());

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        // Repository should have been called 3 times
        assert_eq!(repository.get_operation_count(), 3);
    }

    #[tokio::test]
    async fn test_service_cloning_efficiency() {
        let repository = Arc::new(TestToDoItemRepository::new());
        let original_service = ToDoItemService::new(repository.clone());

        // Clone service multiple times
        let cloned_services: Vec<_> = (0..100).map(|_| original_service.clone()).collect();

        // All cloned services should share the same handler instances
        let original_handler_ptr = Arc::as_ptr(&original_service.get_all_handler());

        for cloned_service in &cloned_services {
            let cloned_handler_ptr = Arc::as_ptr(&cloned_service.get_all_handler());
            assert_eq!(original_handler_ptr, cloned_handler_ptr);
        }

        // Test concurrent execution with cloned services
        let mut tasks = vec![];
        for (i, service) in cloned_services.into_iter().enumerate() {
            let task = tokio::spawn(async move {
                let handler = service.get_all_handler();
                let result = handler.execute().await;
                (i, result)
            });
            tasks.push(task);
        }

        // Wait for all tasks and verify results
        for task in tasks {
            let (task_id, result) = task.await.unwrap();
            assert!(result.is_ok(), "Task {} failed", task_id);
        }

        // All 100 operations should have been recorded
        assert_eq!(repository.get_operation_count(), 100);
    }

    #[tokio::test]
    async fn test_concurrent_handler_access() {
        let repository = Arc::new(TestToDoItemRepository::new());
        let service = Arc::new(ToDoItemService::new(repository.clone()));

        // Add test data
        let test_item = ToDoItem::new(
            "Concurrent Test".to_string(),
            "Testing concurrency".to_string(),
        );
        repository.items.lock().unwrap().push(test_item);

        // Create many concurrent tasks accessing different handlers
        let mut tasks = vec![];

        for i in 0..50 {
            let service_clone = service.clone();
            let task = tokio::spawn(async move {
                match i % 4 {
                    0 => {
                        let handler = service_clone.get_all_handler();
                        handler.execute().await.map(|items| items.len())
                    }
                    1 => {
                        let handler = service_clone.create_handler();
                        let query = application::CreateToDoItemQuery::new(
                            &format!("Task {}", i),
                            &format!("Note {}", i),
                        );
                        handler.execute(query).await.map(|_| 1)
                    }
                    2 => {
                        let handler = service_clone.get_all_handler();
                        handler.execute().await.map(|items| items.len())
                    }
                    _ => {
                        let handler = service_clone.get_all_handler();
                        handler.execute().await.map(|items| items.len())
                    }
                }
            });
            tasks.push(task);
        }

        // Wait for all tasks to complete
        let mut success_count = 0;
        for task in tasks {
            if task.await.unwrap().is_ok() {
                success_count += 1;
            }
        }

        assert_eq!(success_count, 50);
        assert!(repository.get_operation_count() > 0);
    }

    #[tokio::test]
    async fn test_memory_leak_prevention() {
        let repository = Arc::new(TestToDoItemRepository::new());

        // Create many services and let them go out of scope
        for _ in 0..1000 {
            let service = ToDoItemService::new(repository.clone());
            let handler = service.get_all_handler();

            // Use the handler briefly
            let _ = handler.execute().await;

            // Service and handler should be dropped here
        }

        // Repository should still be accessible and functional
        assert_eq!(Arc::strong_count(&repository), 1);
        assert_eq!(repository.get_operation_count(), 1000);
    }

    #[tokio::test]
    async fn test_boxed_vs_arc_performance() {
        let repository = Arc::new(TestToDoItemRepository::new());

        // Test Arc-based service
        repository.reset_count();
        let arc_service = ToDoItemService::new(repository.clone());

        let arc_start = std::time::Instant::now();
        for _ in 0..100 {
            let handler = arc_service.get_all_handler();
            let _ = handler.execute().await;
        }
        let arc_duration = arc_start.elapsed();
        let arc_operations = repository.get_operation_count();

        // Test Box-based service
        repository.reset_count();
        let box_service = application::ToDoItemServiceBoxed::new(repository.clone());

        let box_start = std::time::Instant::now();
        for _ in 0..100 {
            let handler = box_service.create_get_all_handler();
            let _ = handler.execute().await;
        }
        let box_duration = box_start.elapsed();
        let box_operations = repository.get_operation_count();

        // Both should complete all operations
        assert_eq!(arc_operations, 100);
        assert_eq!(box_operations, 100);

        // Arc-based should generally be faster due to shared instances
        // (though this test might be too small to see significant difference)
        println!(
            "Arc duration: {:?}, Box duration: {:?}",
            arc_duration, box_duration
        );
    }

    #[tokio::test]
    async fn test_service_send_sync_compliance() {
        let repository = Arc::new(TestToDoItemRepository::new());
        let service = ToDoItemService::new(repository);

        // Test that service can be sent across thread boundaries
        let service_arc = Arc::new(service);

        let handle = tokio::spawn(async move {
            let handler = service_arc.get_all_handler();
            handler.execute().await
        });

        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // Helper function to verify thread safety at compile time
    fn _verify_send_sync<T: Send + Sync>(_: T) {}

    #[test]
    fn test_compile_time_send_sync() {
        let repository = Arc::new(TestToDoItemRepository::new());
        let service = ToDoItemService::new(repository.clone());

        // These should compile without errors, proving Send + Sync compliance
        _verify_send_sync(service.clone());
        _verify_send_sync(service.get_handler());
        _verify_send_sync(repository);
    }
}
