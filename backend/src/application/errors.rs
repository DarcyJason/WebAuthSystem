use thiserror::Error;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("domain error")]
    DomainError,
    #[error("infrastructure error")]
    InfrastructureError,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user not found")]
    UsernotFound,
    #[error("user already exists")]
    UserAlreadyExists,
}
