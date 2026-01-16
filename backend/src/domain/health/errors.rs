use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Internal server error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("SurrealDB is unhealthy")]
    SurrealDBIsUnhealthy
}
