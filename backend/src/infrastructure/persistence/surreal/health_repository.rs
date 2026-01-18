use crate::domain::error::{DomainError, RepoResult};
use crate::domain::health::errors::HealthError;
use crate::domain::health::repositories::HealthRepository;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct SurrealHealthRepository {}

impl SurrealHealthRepository {
    pub fn new() -> Self {
        SurrealHealthRepository {}
    }
}

impl Default for SurrealHealthRepository {
    fn default() -> Self {
        SurrealHealthRepository::new()
    }
}

#[async_trait]
impl HealthRepository for SurrealHealthRepository {
    async fn check(&self) -> RepoResult<()> {
        let result = reqwest::Client::new()
            .get("http://localhost:10086/health")
            .send()
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(DomainError::Validation(
                HealthError::SurrealDBIsUnhealthy.to_string(),
            ))
        }
    }
}
