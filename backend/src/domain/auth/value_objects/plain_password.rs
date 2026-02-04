use serde::{Deserialize, Serialize};

pub enum PlainPasswordError {
    PasswordRequired,
    PasswordTooShort,
    PasswordTooLong,
    PasswordMissingDigit,
    PasswordMissingLowerCase,
    PasswordMissingUpperCase,
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
