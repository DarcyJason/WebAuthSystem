use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("Respository is unavailable")]
    RespoitoryUnavailable,
}