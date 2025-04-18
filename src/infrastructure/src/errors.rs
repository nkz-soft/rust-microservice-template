use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("item with id {id} not found")]
    ItemNotFound { id: Uuid },

    #[error("internal error: {0}")]
    InternalError(String),
}
