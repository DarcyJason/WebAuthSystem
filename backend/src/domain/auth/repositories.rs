use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::errors::InfraResult;
use async_trait::async_trait;
use surrealdb::RecordId;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> InfraResult<Option<User>>;
    async fn login(&self, identity: LoginIdentity) -> InfraResult<Option<User>>;
    async fn logout(&self, user_id: &str) -> InfraResult<()>;
    async fn forget_password(&self, email: &str) -> InfraResult<()>;
    async fn reset_password(&self, token: &str, new_password: &str) -> InfraResult<()>;
}

pub trait AuthTokenRepository: Send + Sync {
    fn generate_access_token(&self, user_id: RecordId) -> InfraResult<AccessToken>;
    fn generate_refresh_token(&self) -> InfraResult<RefreshToken>;
    fn verify_access_token(&self, token: &str) -> InfraResult<bool>;
}
