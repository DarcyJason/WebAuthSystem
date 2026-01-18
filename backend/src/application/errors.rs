use crate::domain::error::DomainError;
use thiserror::Error;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] DomainError),
    #[error("infrastructure failure")]
    Infrastructure,
    #[error("unauthorized")]
    Unauthorized,
    #[error("unexpected error")]
    Unexpected,
}
