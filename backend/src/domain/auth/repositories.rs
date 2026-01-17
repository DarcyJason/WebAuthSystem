use crate::domain::error::DomainResult;
use crate::domain::user::entities::User;
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(&self, user: &User) -> DomainResult<()>;
    async fn login(&self, username: &str, password: &str) -> DomainResult<User>;
    async fn logout(&self, user_id: &str) -> DomainResult<()>;
    async fn forget_password(&self, email: &str) -> DomainResult<()>;
    async fn reset_password(&self, token: &str, new_password: &str) -> DomainResult<()>;
}
