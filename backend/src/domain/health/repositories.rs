use async_trait::async_trait;

use crate::domain::health::errors::HealthError;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check(&self) -> Result<(), HealthError>;
}
