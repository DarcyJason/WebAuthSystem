use crate::domain::error::DomainResult;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::{Email, HashPassword, Username};
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<()>;
    async fn login(
        &self,
        username: Option<Username>,
        email: Option<Email>,
        password: String,
    ) -> DomainResult<User>;
    async fn logout(&self, user_id: &str) -> DomainResult<()>;
    async fn forget_password(&self, email: &str) -> DomainResult<()>;
    async fn reset_password(&self, token: &str, new_password: &str) -> DomainResult<()>;
}
