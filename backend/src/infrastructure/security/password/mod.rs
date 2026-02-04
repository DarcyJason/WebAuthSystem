use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::domain::auth::{
    services::password_service::{AuthPasswordService, AuthPasswordServiceError},
    value_objects::{plain_password::PlainPassword, user_password_hash::UserPasswordHash},
};

pub struct PasswordService {}

impl PasswordService {
    pub fn new() -> Self {
        PasswordService {}
    }
}

impl AuthPasswordService for PasswordService {
    fn hash(
        &self,
        plain_password: PlainPassword,
    ) -> Result<UserPasswordHash, AuthPasswordServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(plain_password.value().as_bytes(), &salt)
            .map_err(|_| AuthPasswordServiceError::HashPasswordError)?
            .to_string();
        Ok(UserPasswordHash::new(password_hash))
    }
    fn compare(
        &self,
        plain_password: PlainPassword,
        hashed_password: UserPasswordHash,
    ) -> Result<bool, AuthPasswordServiceError> {
        let parsed_hash = PasswordHash::new(hashed_password.value())
            .map_err(|_| AuthPasswordServiceError::ParseHashedPasswordError)?;
        let is_matyched = Argon2::default()
            .verify_password(plain_password.value().as_bytes(), &parsed_hash)
            .is_ok();
        Ok(is_matyched)
    }
}
