use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::domain::auth::service::password_service::PasswordService;
use crate::domain::auth::value_objects::credentials::plain_password::PlainPassword;
use crate::domain::auth::value_objects::user::user_password_hash::UserPasswordHash;
use crate::infrastructure::errors::password_service_error::PasswordServiceError;

pub struct PasswordServiceImplementation;

impl PasswordServiceImplementation {
    pub fn new() -> Self {
        PasswordServiceImplementation {}
    }
}

impl Default for PasswordServiceImplementation {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordService for PasswordServiceImplementation {
    fn hash(
        &self,
        plain_password: PlainPassword,
    ) -> Result<UserPasswordHash, PasswordServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(plain_password.value().as_bytes(), &salt)
            .map_err(|_| PasswordServiceError::HashPasswordError)?
            .to_string();
        Ok(UserPasswordHash::new(password_hash))
    }
    fn compare(
        &self,
        plain_password: PlainPassword,
        hashed_password: UserPasswordHash,
    ) -> Result<bool, PasswordServiceError> {
        let parsed_hash = PasswordHash::new(hashed_password.value())
            .map_err(|_| PasswordServiceError::ParseHashedPasswordError)?;
        let is_matyched = Argon2::default()
            .verify_password(plain_password.value().as_bytes(), &parsed_hash)
            .is_ok();
        Ok(is_matyched)
    }
}
