use std::fmt::Display;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};

pub enum HashPasswordError {
    HashPasswordError,
    ParseHashedPassordError,
    InvalidCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashPassword(String);
impl HashPassword {
    pub fn new(password: String) -> Result<Self, HashPasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| HashPasswordError::HashPasswordError)?
            .to_string();
        Ok(HashPassword(password_hash))
    }
    pub fn verify_password(&self, password: &str) -> Result<bool, HashPasswordError> {
        let parsed_hash =
            PasswordHash::new(&self.0).map_err(|_| HashPasswordError::ParseHashedPassordError)?;
        let is_matyched = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        Ok(is_matyched)
    }
}

impl Display for HashPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
