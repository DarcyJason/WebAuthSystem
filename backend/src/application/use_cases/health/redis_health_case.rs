use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::HealthCache;

pub struct RedisHealthCase<R>
where
    R: HealthCache,
{
    redis_health_cache: R,
}

impl<R> RedisHealthCase<R>
where
    R: HealthCache,
{
    pub fn new(redis_health_cache: R) -> Self {
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
