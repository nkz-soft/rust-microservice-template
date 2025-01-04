use crate::queries::*;
use crate::repositories::*;
use domain::ToDoItem;
use std::rc::Rc;
use uuid::Uuid;

pub struct GetToDoItemQueryHandler {
    repository: Rc<dyn ToDoItemRepository>,
}

impl GetToDoItemQueryHandler {
    pub fn new(repository: Rc<dyn ToDoItemRepository>) -> GetToDoItemQueryHandler {
        GetToDoItemQueryHandler {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, query: GetToDoItemQuery) -> anyhow::Result<ToDoItem> {
        self.repository.get_by_id(query.id.unwrap()).await
    }
}

pub struct GetAllToDoItemQueryHandler {
    repository: Rc<dyn ToDoItemRepository>,
}

impl GetAllToDoItemQueryHandler {
    pub fn new(repository: Rc<dyn ToDoItemRepository>) -> GetAllToDoItemQueryHandler {
        GetAllToDoItemQueryHandler {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<ToDoItem>> {
        self.repository.get_all().await
    }
}

pub struct CreateToDoItemQueryHandler {
    repository: Rc<dyn ToDoItemRepository>,
}

impl CreateToDoItemQueryHandler {
    pub fn new(repository: Rc<dyn ToDoItemRepository>) -> CreateToDoItemQueryHandler {
        CreateToDoItemQueryHandler {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, query: CreateToDoItemQuery) -> anyhow::Result<Uuid> {
        self.repository
            .save(ToDoItem::new(query.title, query.note))
            .await
    }
}

pub struct UpdateToDoItemQueryHandler {
    repository: Rc<dyn ToDoItemRepository>,
}

impl UpdateToDoItemQueryHandler {
    pub fn new(repository: Rc<dyn ToDoItemRepository>) -> UpdateToDoItemQueryHandler {
        UpdateToDoItemQueryHandler {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, query: UpdateToDoItemQuery) -> anyhow::Result<Uuid> {
        self.repository
            .save(ToDoItem::new_id(query.id, query.title, query.note))
            .await
    }
}

pub struct DeleteToDoItemQueryHandler {
    repository: Rc<dyn ToDoItemRepository>,
}

impl DeleteToDoItemQueryHandler {
    pub fn new(repository: Rc<dyn ToDoItemRepository>) -> DeleteToDoItemQueryHandler {
        DeleteToDoItemQueryHandler {
            repository: repository.clone(),
        }
    }

    pub async fn execute(&self, query: DeleteToDoItemQuery) -> anyhow::Result<()> {
        self.repository.delete(query.id).await
    }
}
