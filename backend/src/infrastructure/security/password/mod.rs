use argon2::{
    password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};

use crate::domain::auth::services::password_service::AuthPasswordService;
use crate::domain::{
    auth::value_objects::plain_password::PlainPassword,
    user::value_objects::user_password_hash::UserPasswordHash,
};
use crate::infrastructure::errors::password_service_error::PasswordServiceError;

pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        PasswordService {}
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPasswordService for PasswordService {
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
