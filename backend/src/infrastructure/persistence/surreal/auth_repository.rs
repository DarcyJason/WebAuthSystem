use crate::domain::auth::value_objects::{LoginIdentity, PlainPassword};
use crate::domain::error::DomainError;
use crate::domain::user::value_objects::{Email, HashPassword, Username};
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
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<()> {
        let user = self.user_repo.find_by_username(&username).await?;
        if user.is_some() {
            return Err(DomainError::Repository("User already exists".to_string()));
        }
        let user = self.user_repo.find_by_email(&email).await?;
        if user.is_some() {
            return Err(DomainError::Repository("User already exists".to_string()));
        }
        self.user_repo.save(username, email, hash_password).await?;
        Ok(())
    }

    async fn login(
        &self,
        identity: LoginIdentity,
    ) -> DomainResult<Option<User>> {
        let user = match identity {
            LoginIdentity::Username(username) => self.user_repo.find_by_username(&username).await?,
            LoginIdentity::Email(email) => self.user_repo.find_by_email(&email).await?,
        };
        Ok(user)
    }

    async fn logout(&self, _user_id: &str) -> DomainResult<()> {
        todo!("Implement logout logic")
    }

    async fn forget_password(&self, _email: &str) -> DomainResult<()> {
        todo!("Implement forget password logic")
    }

    async fn reset_password(&self, _token: &str, _new_password: &str) -> DomainResult<()> {
        todo!("Implement reset password logic")
    }
}
