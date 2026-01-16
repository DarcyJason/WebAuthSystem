use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::domain::user::errors::UserError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);
impl Username {
    pub fn new(username: String) -> Result<Self, UserError> {
        let username = username.trim().to_string();
        if username.is_empty() {
            return Err(UserError::UsernameIsrequired);
        }
        if username.len() < 2 {
            return Err(UserError::UsernameIsTooShort);
        }
        if username.len() > 20 {
            return Err(UserError::UsernameIsTooLong);
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
    pub fn new(email: String) -> Result<Self, UserError> {
        let email = email.trim().to_string();
        if email.is_empty() {
            return Err(UserError::EmailIsInvalid);
        }
        if !email.contains("@") {
            return Err(UserError::EmailIsInvalid);
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
    pub fn new(password: String) -> Result<Self, UserError> {
        let password = password.trim().to_string();
        if password.is_empty() {
            return Err(UserError::PasswordIsRequired);
        }
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| UserError::HashPasswordError)?
            .to_string();
        Ok(HashPassword(password_hash))
    }
    pub fn verify_password(hash_password: String, password: String) -> Result<bool, UserError> {
        let parsed_hash =
            PasswordHash::new(&hash_password).map_err(|_| UserError::ParseHashPasswordError)?;
        let password = password.trim().as_bytes();
        let result = Argon2::default()
            .verify_password(password, &parsed_hash)
            .is_ok();
        Ok(result)
    }
}

impl Display for HashPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
