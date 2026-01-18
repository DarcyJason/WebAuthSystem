use serde::Deserialize;

use crate::domain::{
    auth::errors::AuthError,
    error::DomainError,
    user::value_objects::{Email, Username},
};

#[derive(Debug, Clone, Deserialize)]
pub enum LoginIdentity {
    Username(Username),
    Email(Email),
}

impl LoginIdentity {
    pub fn parse(raw: String) -> Result<Self, DomainError> {
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
    pub fn new(raw: String) -> Result<Self, DomainError> {
        let raw = raw.trim();
        if raw.len() < 8 {
            return Err(AuthError::PasswordTooShort.into());
        }
        if raw.len() > 20 {
            return Err(AuthError::PasswordTooLong.into());
        }
        Ok(Self(raw.to_owned()))
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}
