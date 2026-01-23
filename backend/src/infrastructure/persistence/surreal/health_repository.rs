use crate::domain::health::repositories::HealthRepository;
use crate::infrastructure::errors::InfraResult;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
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
    async fn check(&self) -> InfraResult<()> {
        let result = reqwest::Client::new()
            .get("http://localhost:10086/health")
            .send()
            .await
            .map_err(|_| SurrealDBError::SendRequestError)?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(SurrealDBError::ConnectionError.into())
        }
    }
}
