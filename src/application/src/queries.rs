use uuid::Uuid;

pub struct GetToDoItemQuery {
    pub id: Option<Uuid>,
}

impl GetToDoItemQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        GetToDoItemQuery { id }
    }
}

///
pub struct CreateToDoItemQuery {
    pub title: String,
    pub note: String,
}

impl CreateToDoItemQuery {
    pub fn new(title: &String, note: &String) -> Self {
        CreateToDoItemQuery {
            title: title.into(),
            note: note.into(),
        }
    }
}

///
pub struct UpdateToDoItemQuery {
    pub id: Uuid,
    pub title: String,
    pub note: String,
}

impl UpdateToDoItemQuery {
    pub fn new(id: Uuid, title: &String, note: &String) -> Self {
        UpdateToDoItemQuery {
            id,
            title: title.into(),
            note: note.into(),
        }
    }
}

///
pub struct DeleteToDoItemQuery {
    pub id: Uuid,
}

impl DeleteToDoItemQuery {
    pub fn new(id: Uuid) -> Self {
        DeleteToDoItemQuery { id }
    }
}
