use crate::dtos::ToDoItemDto;

pub struct GetToDoItemQueryHandler {
}

impl GetToDoItemQueryHandler {

    pub fn new() -> Self {
        GetToDoItemQueryHandler {
        }
    }

    pub fn execute(&self) -> Result<ToDoItemDto, String> {
        Ok(ToDoItemDto::get())
    }
}
