use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlainPasswordError {
    #[error("password is required")]
    PasswordRequired,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("password is missing digit")]
    PasswordMissingDigit,
    #[error("password is missing lower case letter")]
    PasswordMissingLowerCase,
    #[error("password is missing upper case letter")]
    PasswordMissingUpperCase,
    #[error("password is missing special symbol")]
    PasswordMissingSpetial,
}
