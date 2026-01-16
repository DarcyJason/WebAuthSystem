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
}
