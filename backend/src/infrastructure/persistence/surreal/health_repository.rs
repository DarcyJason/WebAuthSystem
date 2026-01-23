use crate::infrastructure::errors::InfraResult;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;

#[derive(Debug, Clone)]
pub struct SurrealHealthRepository {}

impl SurrealHealthRepository {
    pub fn new() -> Self {
        SurrealHealthRepository {}
    }
    pub async fn check(&self) -> InfraResult<()> {
        let result = reqwest::Client::new()
            .get("http://localhost:10086/health")
            .send()
            .await
            .map_err(|_| SurrealDBError::RequestHealthEndpointError)?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(SurrealDBError::ConnectionError.into())
        }
    }
}

impl Default for SurrealHealthRepository {
    fn default() -> Self {
        SurrealHealthRepository::new()
    }
}
