use crate::{
    domain::{
        auth::value_objects::credentials::plain_password::PlainPassword,
        user::entities::user::user_password_hash::UserPasswordHash,
    },
    infrastructure::errors::password_service_error::PasswordServiceError,
};

pub trait AuthPasswordService: Send + Sync {
    fn hash(&self, plain_password: PlainPassword)
    -> Result<UserPasswordHash, PasswordServiceError>;
    fn compare(
        &self,
        plain_password: PlainPassword,
        hashed_password: UserPasswordHash,
    ) -> Result<bool, PasswordServiceError>;
}
