use crate::domain::auth::services::password_service::PasswordService;
use crate::domain::error::{
    DomainResult, HashPasswordFailedSnafu, ParsedHashedPasswordFailedSnafu,
};
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::credential::plain_password::PlainPassword;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub struct Argon2PasswordService;

impl Argon2PasswordService {
    pub fn new() -> Self {
        Argon2PasswordService
    }
}

impl Default for Argon2PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordService for Argon2PasswordService {
    fn hash_password(&self, plain_password: PlainPassword) -> DomainResult<PasswordCredential> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(plain_password.value().as_bytes(), &salt)
            .map_err(|e| {
                HashPasswordFailedSnafu {
                    message: e.to_string(),
                }
                .build()
            })?
            .to_string();
        Ok(PasswordCredential::new(password_hash))
    }
    fn verify_password(
        &self,
        hashed_password: PasswordCredential,
        plain_password: PlainPassword,
    ) -> DomainResult<bool> {
        let hashed_password_str = hashed_password.value();
        let parsed_hash_password = PasswordHash::new(&hashed_password_str).map_err(|e| {
            {
                ParsedHashedPasswordFailedSnafu {
                    message: e.to_string(),
                }
            }
            .build()
        })?;
        let is_matyched = Argon2::default()
            .verify_password(plain_password.value().as_bytes(), &parsed_hash_password)
            .is_ok();
        Ok(is_matyched)
    }
}
