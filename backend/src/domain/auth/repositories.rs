use async_trait::async_trait;
use crate::domain::user::entities::User;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(&self, user: &User) -> Result<(), anyhow::Error>;
    async fn login(&self, username: &str, password: &str) -> Result<User, anyhow::Error>;
    async fn logout(&self, user_id: &str) -> Result<(), anyhow::Error>;
    async fn forget_password(&self, email: &str) -> Result<(), anyhow::Error>;
    async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), anyhow::Error>;
}
