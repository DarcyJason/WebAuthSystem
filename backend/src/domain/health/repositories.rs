use async_trait::async_trait;

use crate::infrastructure::errors::InfraResult;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check(&self) -> InfraResult<()>;
}

#[async_trait]
pub trait HealthCache: Send + Sync {
    async fn check(&self) -> InfraResult<()>;
}
