use crate::domain::errors::{DomainError, RepoResult};
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
            .map_err(|_| DomainError::RepositoryError)?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(DomainError::RepositoryError)
        }
    }
}
