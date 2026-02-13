use thiserror::Error;

pub type DomainUserResult<T> = Result<T, DomainUserError>;

#[derive(Debug, Error)]
pub enum DomainUserError {
    #[error("username is invalid")]
    UsernameInvalid,
    #[error("password is too short")]
    UsernameRequired,
    #[error("username is too long")]
    UsernameTooLong,
    #[error("hash password error")]
    HashPasswordError,
    #[error("parse hashed password error")]
    ParseHashedPassordError,
    #[error("email is required")]
    EmailRequired,
    #[error("email is invalid")]
    EmailInvalid,
}
