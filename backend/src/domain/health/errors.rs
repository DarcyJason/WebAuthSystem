use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("request surrealdb health endpoint error")]
    RequestSurrealDBHealthEndpointError,
    #[error("surrealdb connection error")]
    SurrealDBConnectionError,
    #[error("request redis health endpoint error")]
    RequestRedisHealthEndpointError,
    #[error("redis connection error")]
    RedisConnectionError,
}
