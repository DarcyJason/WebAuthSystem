use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password is required")]
    PasswordIsRequired,
    #[error("password is too short")]
    PasswordIsTooShort,
    #[error("password is too long")]
    PasswordIsTooLong,
    #[error("passwords not match")]
    PasswordsNotMatch,
    #[error("invalid credentials")]
    InvalidCredentials,
}
