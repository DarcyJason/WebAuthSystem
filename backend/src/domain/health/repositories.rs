use async_trait::async_trait;

use crate::domain::error::RepoResult;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check(&self) -> RepoResult<()>;
}
