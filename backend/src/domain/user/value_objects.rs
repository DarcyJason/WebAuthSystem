use crate::domain::auth::errors::AuthError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::domain::{error::DomainError, user::errors::UserError};
use crate::domain::auth::value_objects::PlainPassword;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);
impl Username {
    pub fn new(username: String) -> Result<Self, DomainError> {
        let username = username.trim().to_string();
        if username.is_empty() {
            return Err(UserError::UsernameIsrequired.into());
        }
        if username.contains("@") {
            return Err(UserError::UsernameIsInvalid.into());
        }
        if username.len() < 2 {
            return Err(UserError::UsernameIsTooShort.into());
        }
        if username.len() > 20 {
            return Err(UserError::UsernameIsTooLong.into());
        }
        Ok(Username(username))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        let email = email.trim().to_string();
        if email.is_empty() {
            return Err(UserError::EmailIsRequired.into());
        }
        if !email.contains("@") {
            return Err(UserError::EmailIsInvalid.into());
        }
        Ok(Email(email))
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashPassword(String);
impl HashPassword {
    pub fn new(password: String) -> Result<Self, DomainError> {
        let password = password.trim().to_string();
        if password.is_empty() {
            return Err(UserError::PasswordIsRequired.into());
        }
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| UserError::HashPasswordError)?
            .to_string();
        Ok(HashPassword(password_hash))
    }
    pub fn verify_password(&self, password: &PlainPassword) -> Result<(), DomainError> {
        let parsed_hash =
            PasswordHash::new(&self.0)
                .map_err(|_| UserError::ParseHashPasswordError)?;
        Argon2::default()
            .verify_password(password.expose().as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)?;
        Ok(())
    }
}

impl Display for HashPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
