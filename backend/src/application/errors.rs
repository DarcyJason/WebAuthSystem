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
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("hash password error")]
    HashPasswordError,
    #[error("parse hashed password error")]
    ParseHashedPasswordError,
    #[error("generate access_token error")]
    GenerateAccessTokenError,
    #[error("generate refresh_token error")]
    GenerateRefreshTokenError,
    #[error("access token is invalid")]
    AccessTokenInvalid,
}
