use crate::domain::{
    auth::value_objects::plain_password::PlainPassword,
    user::value_objects::user_password_hash::UserPasswordHash,
};

pub enum AuthPasswordServiceError {
    HashPasswordError,
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
