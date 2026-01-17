use crate::{
    domain::{
        auth::repositories::AuthRepository,
        error::DomainResult,
        user::{entities::User, repositories::UserRepository},
    },
    infrastructure::persistence::surreal::user_repository::SurrealUserRepository,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct SurrealAuthRepository {
    user_repo: SurrealUserRepository,
}

impl SurrealAuthRepository {
    pub fn new(user_repo: SurrealUserRepository) -> Self {
        SurrealAuthRepository { user_repo }
    }
}

#[async_trait]
impl AuthRepository for SurrealAuthRepository {
    async fn register(&self, user: &User) -> DomainResult<()> {
        self.user_repo.save(user).await?;
        Ok(())
    }

    async fn login(&self, username: &str, password: &str) -> DomainResult<User> {
        todo!("Implement login logic")
    }

    async fn logout(&self, user_id: &str) -> DomainResult<()> {
        todo!("Implement logout logic")
    }

    async fn forget_password(&self, email: &str) -> DomainResult<()> {
        todo!("Implement forget password logic")
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> DomainResult<()> {
        todo!("Implement reset password logic")
    }
}
