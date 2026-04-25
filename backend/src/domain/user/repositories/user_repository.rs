use crate::domain::error::DomainResult;
use crate::domain::user::aggregates::user::UserEntity;
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::domain::user::value_objects::user::user_name::UserName;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use async_trait::async_trait;

#[async_trait]
pub trait UserCommandRepository: Send + Sync {
    async fn save(&self, user: &UserEntity) -> DomainResult<UserEntity>;
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<UserEntity>;
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<UserEntity>;
    async fn update_access_token_version(
        &self,
        user_id: &UserId,
        access_token_version: &AccessTokenVersion,
    ) -> DomainResult<UserEntity>;
}

#[async_trait]
pub trait UserQueryRepository: Send + Sync {
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<UserEntity>>;
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<UserEntity>>;
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<UserEntity>>;
    async fn get_by_name_or_email(
        &self,
        user_name: &Option<UserName>,
        user_email: &Option<UserEmail>,
    ) -> DomainResult<Option<UserEntity>>;
}
