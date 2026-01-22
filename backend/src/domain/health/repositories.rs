use async_trait::async_trait;

use crate::domain::errors::RepoResult;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check(&self) -> RepoResult<()>;
}

#[async_trait]
pub trait HealthCache: Send + Sync {
    async fn check(&self) -> RepoResult<()>;
}
