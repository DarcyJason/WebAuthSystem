use thiserror::Error;

pub type RepoResult<T> = Result<T, InfrastructureError>;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("record not found")]
    NotFound,
    #[error("unique constraint violated")]
    Conflict,
    #[error("network error")]
    Network,
}
