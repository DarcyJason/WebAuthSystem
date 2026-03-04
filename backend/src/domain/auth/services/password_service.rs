use crate::domain::{
    auth::value_objects::plain_password::PlainPassword,
    user::value_objects::user_password_hash::UserPasswordHash,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthPasswordServiceError {
    #[error("hash password error")]
    HashPasswordError,
    #[error("parse hashed password error")]
    ParseHashedPasswordError,
}

pub trait AuthPasswordService: Send + Sync {
    fn hash(
        &self,
        plain_password: PlainPassword,
    ) -> Result<UserPasswordHash, AuthPasswordServiceError>;
    fn compare(
        &self,
        plain_password: PlainPassword,
        hashed_password: UserPasswordHash,
    ) -> Result<bool, AuthPasswordServiceError>;
}
