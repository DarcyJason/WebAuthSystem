use crate::domain::user::entities::user::User;
use crate::domain::user::value_objects::{
    user_email::UserEmail, user_id::UserId, user_name::UserName,
};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("storage is unavailable")]
    StorageUnavailable,
    #[error("persistence operation failed")]
    PersistenceFailed,
    #[error("failed to deserialize stored user data")]
    DeserializationFailed,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_name(&self, user_name: &UserName)
    -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_name_or_email(
        &self,
        user_name: &UserName,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn update_status_as_true(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError>;
}
