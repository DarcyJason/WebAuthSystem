use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Internal server error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("SurrealDB is unhealthy")]
    SurrealDBIsUnhealthy,
    #[error("Redis is unhealthy")]
    RedisIsUnhealthy,
}
