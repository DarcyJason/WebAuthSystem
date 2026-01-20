use serde::Deserialize;

pub enum PlainPasswordError {
    PasswordRequired,
    PasswordTooShort,
    PasswordTooLong,
    PasswordMissingDigit,
    PasswordMissingLowerCase,
    PasswordMissingUpperCase,
    PasswordMissingSpetial,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(raw: String) -> Result<Self, PlainPasswordError> {
        if raw.is_empty() {
            return Err(PlainPasswordError::PasswordRequired);
        }
        if raw.len() < 8 {
            return Err(PlainPasswordError::PasswordTooShort);
        }
        if raw.len() > 16 {
            return Err(PlainPasswordError::PasswordTooLong);
        }
        if !raw.chars().any(|c| c.is_ascii_digit()) {
            return Err(PlainPasswordError::PasswordMissingDigit);
        }
        if !raw.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(PlainPasswordError::PasswordMissingLowerCase);
        }
        if !raw.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(PlainPasswordError::PasswordMissingUpperCase);
        }
        if !raw.chars().any(|c| "!@#$%^&*".contains(c)) {
            return Err(PlainPasswordError::PasswordMissingSpetial);
        }
        Ok(Self(raw.to_owned()))
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}
