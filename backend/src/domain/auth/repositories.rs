use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::error::RepoResult;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> RepoResult<Option<User>>;
    async fn login(&self, identity: LoginIdentity) -> RepoResult<Option<User>>;
    async fn logout(&self, user_id: &str) -> RepoResult<()>;
    async fn forget_password(&self, email: &str) -> RepoResult<()>;
    async fn reset_password(&self, token: &str, new_password: &str) -> RepoResult<()>;
}
