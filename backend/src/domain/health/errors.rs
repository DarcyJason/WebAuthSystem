use std::fmt::Display;

pub enum HealthError {
    RequestSurrealDBHealthEndpointError,
    SurrealDBConnectionError,
    RequestRedisHealthEndpointError,
    RedisConnectionError,
}

impl Display for HealthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthError::RequestSurrealDBHealthEndpointError => write!(f, "Failed to request SurrealDB health endpoint"),
            HealthError::SurrealDBConnectionError => write!(f, "SurrealDB connection error"),
            HealthError::RequestRedisHealthEndpointError => write!(f, "Failed to request Redis health endpoint"),
            HealthError::RedisConnectionError => write!(f, "Redis connection error"),
        }
    }
}
