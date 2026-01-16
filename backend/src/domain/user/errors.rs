use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("username is required")]
    UsernameIsrequired,
    #[error("username is too short")]
    UsernameIsTooShort,
    #[error("username is too long")]
    UsernameIsTooLong,
    #[error("email is required")]
    EmailIsRequired,
    #[error("email is invalid")]
    EmailIsInvalid,
    #[error("password is required")]
    PasswordIsRequired,
    #[error("password is invalid")]
    PasswordIsInvalid,
    #[error("hash password error")]
    HashPasswordError,
    #[error("parse hash_password error")]
    ParseHashPasswordError,
    #[error("passwords not match")]
    PasswordsNotMatch,
}
