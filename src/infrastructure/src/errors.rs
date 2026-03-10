use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("item with id {id} not found")]
    ItemNotFound { id: Uuid },

    #[error("item with id {id} has a stale version: expected {expected_version}, actual {actual_version}")]
    VersionConflict {
        id: Uuid,
        expected_version: i32,
        actual_version: i32,
    },

    #[error("internal error: {0}")]
    InternalError(String),
}
