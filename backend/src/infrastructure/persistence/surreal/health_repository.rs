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
    async fn check(&self) -> Result<(), HealthError> {
        let result = reqwest::Client::new()
            .get("http://localhost:10086/health")
            .send()
            .await?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(HealthError::SurrealDBIsUnhealthy)
        }
    }
}
