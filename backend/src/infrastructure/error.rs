use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type InfraResult<T> = Result<T, InfrastructureError>;

impl From<InfrastructureError> for DomainError {
    fn from(err: InfrastructureError) -> Self {
        match err {
            InfrastructureError::NotFound(msg) => DomainError::NotFound(msg),
            _ => DomainError::Repository(err.to_string()),
        }
    }
}
