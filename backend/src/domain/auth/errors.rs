use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password is too short")]
    PasswordIsTooShort,
    #[error("password is too long")]
    PasswordIsTooLong,
    #[error("invalid credentials")]
    InvalidCredentials,
}
