use async_trait::async_trait;

use crate::domain::auth::{
    entities::user::User,
    value_objects::{user_email::UserEmail, user_id::UserId, user_name::UserName},
};

pub enum UserRepositoryError {
    StorageUnavailable,
    PersistFailed,
    DataCorrupted,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save_user(&self, user: User) -> Result<Option<User>, UserRepositoryError>;
    async fn find_user_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_user_by_name(
        &self,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_user_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_user_by_name_or_email(
        &self,
        user_name: &UserName,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError>;
}
