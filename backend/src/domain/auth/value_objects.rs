use serde::Deserialize;

use crate::domain::{
    auth::errors::AuthError,
    error::{DomainError, DomainResult},
    user::value_objects::{Email, Username},
};

#[derive(Debug, Clone, Deserialize)]
pub enum LoginIdentity {
    Username(Username),
    Email(Email),
}

impl LoginIdentity {
    pub fn parse(raw: String) -> DomainResult<Self> {
        if raw.contains("@") {
            Ok(Self::Email(Email::new(raw)?))
        } else {
            Ok(Self::Username(Username::new(raw)?))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(raw: String) -> DomainResult<Self> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(DomainError::Validation(
                AuthError::PasswordIsRequired.to_string(),
            ));
        }
        if raw.len() < 8 {
            return Err(DomainError::Validation(
                AuthError::PasswordIsTooShort.to_string(),
            ));
        }
        if raw.len() > 20 {
            return Err(DomainError::Validation(
                AuthError::PasswordIsTooLong.to_string(),
            ));
        }
        Ok(Self(raw.to_owned()))
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}
