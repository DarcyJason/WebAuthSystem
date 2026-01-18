use crate::domain::error::DomainError;
use crate::domain::health::errors::HealthError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] DomainError),
    #[error(transparent)]
    HealthError(#[from] HealthError),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
