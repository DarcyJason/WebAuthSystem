use crate::domain::error::DomainResult;
use crate::domain::identities::aggregates::user::User;
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::user::user_name::UserName;
use crate::domain::identities::value_objects::user::user_status::UserStatus;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> DomainResult<User>;
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<User>>;
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<User>>;
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<User>>;
    async fn update_status(&self, user_id: &UserId, user_status: &UserStatus)
    -> DomainResult<User>;
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<User>;
}
