use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use tracing::error;

#[derive(Debug, Clone)]
pub struct SurrealHealthRepository {}

impl SurrealHealthRepository {
    pub fn new() -> Self {
        SurrealHealthRepository {}
    }
    pub async fn check(&self) -> Result<(), SurrealDBError> {
        let result = reqwest::Client::new()
            .get("http://localhost:10086/health")
            .send()
            .await
            .map_err(|e| {
                error!("request surrealdb health endpoint error: {:?}", e);
                SurrealDBError::RequestHealthEndpointError
            })?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(SurrealDBError::ConnectionError)
        }
    }
}

impl Default for SurrealHealthRepository {
    fn default() -> Self {
        SurrealHealthRepository::new()
    }
}
