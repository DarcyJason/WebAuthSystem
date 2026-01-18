use thiserror::Error;

use crate::domain::{auth::errors::AuthError, user::errors::UserError};

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
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
}

pub type DomainResult<T> = Result<T, DomainError>;
