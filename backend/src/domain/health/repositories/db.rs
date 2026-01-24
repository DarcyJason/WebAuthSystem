use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::health::errors::HealthError;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use crate::infrastructure::persistence::surreal::health_repository::SurrealHealthRepository;
use async_trait::async_trait;

#[async_trait]
pub trait HealthRepository: Send + Sync + 'static {
    async fn check(&self) -> DomainResult<()>;
}

pub struct SurrealHealthRepositoryAdapter {
    inner: SurrealHealthRepository,
}

impl SurrealHealthRepositoryAdapter {
    pub fn new(inner: SurrealHealthRepository) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl HealthRepository for SurrealHealthRepositoryAdapter {
    async fn check(&self) -> DomainResult<()> {
        self.inner.check().await.map_err(|e| match e {
            SurrealDBError::RequestHealthEndpointError => {
                DomainError::HealthError(HealthError::RequestSurrealDBHealthEndpointError)
            }
            SurrealDBError::ConnectionError => {
                DomainError::HealthError(HealthError::SurrealDBConnectionError)
            }
            _ => DomainError::SurrealDBUnavailable,
        })
    }
}
