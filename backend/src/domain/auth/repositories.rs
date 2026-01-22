use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::errors::RepoResult;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use async_trait::async_trait;
use surrealdb::RecordId;

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

pub trait AuthTokenRepository: Send + Sync {
    fn generate_access_token(&self, user_id: RecordId) -> RepoResult<AccessToken>;
    fn generate_refresh_token(&self) -> RepoResult<RefreshToken>;
    fn verify_access_token(&self, token: &str) -> RepoResult<bool>;
}
