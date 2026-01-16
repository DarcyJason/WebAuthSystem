use async_trait::async_trait;
use crate::{
    domain::{
        auth::repositories::AuthRepository,
        user::{entities::User, repositories::UserRepository},
    },
    infrastructure::persistence::surreal::user_repository::SurrealUserRepository,
};

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
    async fn register(&self, user: &User) -> Result<(), anyhow::Error> {
        self.user_repo.save(user).await?;
        Ok(())
    }

    async fn login(&self, username: &str, password: &str) -> Result<User, anyhow::Error> {
        todo!("Implement login logic")
    }

    async fn logout(&self, user_id: &str) -> Result<(), anyhow::Error> {
        todo!("Implement logout logic")
    }

    async fn forget_password(&self, email: &str) -> Result<(), anyhow::Error> {
        todo!("Implement forget password logic")
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), anyhow::Error> {
        todo!("Implement reset password logic")
    }
}
