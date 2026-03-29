use thiserror::Error;
use uuid::Uuid;

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("todo item with id {id} not found")]
    NotFound { id: Uuid },

    #[error(
        "todo item with id {id} has a stale version: expected {expected_version}, actual {actual_version}"
    )]
    Conflict {
        id: Uuid,
        expected_version: i32,
        actual_version: i32,
    },

    #[error("{message}")]
    Internal { message: String },
}

impl ApplicationError {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}
