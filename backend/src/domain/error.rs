use crate::domain::user::errors::UserError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("User error: {0}")]
    User(#[from] UserError),
}

pub type DomainResult<T> = Result<T, DomainError>;
