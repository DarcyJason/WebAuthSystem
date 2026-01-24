pub enum HealthError {
    RequestSurrealDBHealthEndpointError,
    SurrealDBConnectionError,
    RequestRedisHealthEndpointError,
    RedisConnectionError,
}
