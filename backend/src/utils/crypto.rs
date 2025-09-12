use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

use crate::errors::app_error::{AppError, AppResult};

pub fn hash_password(password: String) -> AppResult<String> {
    if password.is_empty() {
        return Err(AppError::PasswordEmpty);
    }
    if password.len() > 64 {
        return Err(AppError::PasswordIsTooLong);
    }
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(AppError::PasswordHashError)?
        .to_string();
    Ok(hashed_password)
}

pub fn compare_hashed_password(password: String, hashed_password: String) -> AppResult<bool> {
    if password.is_empty() {
        return Err(AppError::PasswordEmpty);
    }
    if password.len() > 64 {
        return Err(AppError::PasswordIsTooLong);
    }
    let parsed_hash = PasswordHash::new(&hashed_password).map_err(AppError::PasswordHashError)?;
    let is_password_match = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok_and(|_| true);
    Ok(is_password_match)
}
