use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

pub type RepoResult<T> = Result<T, DomainError>;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("invariant violated: {0}")]
    Invariant(String),
    #[error("repository error: {0}")]
    Repository(String),
}
