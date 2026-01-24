use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::cache::HealthCache;

pub struct RedisHealthCase<RC>
where
    RC: HealthCache,
{
    redis_health_cache: RC,
}

impl<RC> RedisHealthCase<RC>
where
    RC: HealthCache,
{
    pub fn new(redis_health_cache: RC) -> Self {
        RedisHealthCase { redis_health_cache }
    }
    pub async fn execute(&self) -> Result<(), ApplicationError> {
        self.redis_health_cache
            .check()
            .await
            .map_err(|_| ApplicationError::InfrastructureError)?;
        Ok(())
    }
}
