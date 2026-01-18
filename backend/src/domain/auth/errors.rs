use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password too shot")]
    PasswordTooShort,
    #[error("password too long")]
    PasswordTooLong,
    #[error("invalid credentials")]
    InvalidCredentials,
}
