use async_trait::async_trait;

use crate::domain::user::entities::user::User;
use crate::domain::user::repositories::user_repository::{UserRepository, UserRepositoryError};
use crate::domain::user::value_objects::{
    user_email::UserEmail, user_id::UserId, user_name::UserName,
};

pub struct MokaUserRepository;

impl MokaUserRepository {
    pub fn new() -> Self {
        MokaUserRepository
    }
}

impl Default for MokaUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for MokaUserRepository {
    async fn save(&self, _user: User) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::PersistFailed)
    }

    async fn find_by_id(&self, _user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::StorageUnavailable)
    }

    async fn find_by_name(
        &self,
        _user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::StorageUnavailable)
    }

    async fn find_by_email(
        &self,
        _user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::StorageUnavailable)
    }

    async fn find_by_name_or_email(
        &self,
        _user_name: &UserName,
        _user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::StorageUnavailable)
    }
}
