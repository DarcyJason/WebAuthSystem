use crate::domain::{auth::errors::AuthError, error::DomainResult};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::domain::auth::value_objects::PlainPassword;
use crate::domain::{error::DomainError, user::errors::UserError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);
impl Username {
    pub fn new(username: String) -> DomainResult<Self> {
        let username = username.trim().to_string();
        if username.is_empty() {
            return Err(DomainError::Validation(
                UserError::UsernameIsrequired.to_string(),
            ));
        }
        if username.contains("@") {
            return Err(DomainError::Validation(
                UserError::UsernameIsInvalid.to_string(),
            ));
        }
        if username.len() < 2 {
            return Err(DomainError::Validation(
                UserError::UsernameIsTooShort.to_string(),
            ));
        }
        if username.len() > 20 {
            return Err(DomainError::Validation(
                UserError::UsernameIsTooLong.to_string(),
            ));
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
    pub fn new(email: String) -> DomainResult<Self> {
        let email = email.trim().to_string();
        if email.is_empty() {
            return Err(DomainError::Validation(
                UserError::EmailIsRequired.to_string(),
            ));
        }
        if !email.contains("@") {
            return Err(DomainError::Validation(
                UserError::EmailIsInvalid.to_string(),
            ));
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
    pub fn new(password: String) -> DomainResult<Self> {
        let password = password.trim().to_string();
        if password.is_empty() {
            return Err(DomainError::Validation(
                UserError::PasswordIsRequired.to_string(),
            ));
        }
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| DomainError::Validation(UserError::HashPasswordError.to_string()))?
            .to_string();
        Ok(HashPassword(password_hash))
    }
    pub fn verify_password(&self, password: &PlainPassword) -> Result<(), DomainError> {
        let parsed_hash = PasswordHash::new(&self.0)
            .map_err(|_| DomainError::Validation(UserError::ParseHashPasswordError.to_string()))?;
        Argon2::default()
            .verify_password(password.expose().as_bytes(), &parsed_hash)
            .map_err(|_| DomainError::Validation(AuthError::InvalidCredentials.to_string()))?;
        Ok(())
    }
}

impl Display for HashPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
