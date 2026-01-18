use crate::application::errors::ApplicationError;
use crate::domain::health::repositories::HealthRepository;

pub struct RedisHealthCase<R>
where
    R: HealthRepository,
{
    redis_health_repo: R,
}

impl<R> RedisHealthCase<R>
where
    R: HealthRepository,
{
    pub fn new(redis_health_repo: R) -> Self {
        RedisHealthCase { redis_health_repo }
    }
    pub async fn execute(&self) -> Result<(&str, ()), ApplicationError> {
        self.redis_health_repo
            .check()
            .await
            .map_err(|_| ApplicationError::Infrastructure)?;
        Ok(("Redis is healthy", ()))
    }
}
