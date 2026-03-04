use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(plain_password: impl Into<String>) -> Result<Self, PlainPasswordError> {
        let plain_password = plain_password.into();
        if plain_password.is_empty() {
            return Err(PlainPasswordError::PasswordRequired);
        }
        if plain_password.len() < 8 {
            return Err(PlainPasswordError::PasswordTooShort);
        }
        if plain_password.len() > 16 {
            return Err(PlainPasswordError::PasswordTooLong);
        }
        if !plain_password.chars().any(|c| c.is_ascii_digit()) {
            return Err(PlainPasswordError::PasswordMissingDigit);
        }
        if !plain_password.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(PlainPasswordError::PasswordMissingLowerCase);
        }
        if !plain_password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(PlainPasswordError::PasswordMissingUpperCase);
        }
        if !plain_password.chars().any(|c| "!@#$%^&*".contains(c)) {
            return Err(PlainPasswordError::PasswordMissingSpetial);
        }
        Ok(Self(plain_password))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
