use crate::domain::auth::errors::AuthError;
use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::error::RepoResult;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::domain::{
    auth::repositories::AuthRepository,
    user::{entities::User, repositories::UserRepository},
};
use crate::infrastructure::persistence::surreal::client::SurrealClient;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct SurrealAuthRepository {
    surreal: SurrealClient,
}

impl SurrealAuthRepository {
    pub fn new(surreal: SurrealClient) -> Self {
        SurrealAuthRepository { surreal }
    }
}

#[async_trait]
impl AuthRepository for SurrealAuthRepository {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> RepoResult<Option<User>> {
        let user = self
            .surreal
            .user_repo()
            .find_by_username_or_email(&username, &email)
            .await?;
        if user.is_some() {
            return Err(AuthError::UserAlreadyExists.into());
        }
        let user = self
            .surreal
            .user_repo()
            .save(username, email, hash_password)
            .await?;
        Ok(user)
    }

    async fn login(&self, identity: LoginIdentity) -> RepoResult<Option<User>> {
        let user = match identity {
            LoginIdentity::Username(username) => {
                self.surreal.user_repo().find_by_username(&username).await?
            }
            LoginIdentity::Email(email) => self.surreal.user_repo().find_by_email(&email).await?,
        };
        Ok(user)
    }

    async fn logout(&self, _user_id: &str) -> RepoResult<()> {
        todo!("Implement logout logic")
    }

    async fn forget_password(&self, _email: &str) -> RepoResult<()> {
        todo!("Implement forget password logic")
    }

    async fn reset_password(&self, _token: &str, _new_password: &str) -> RepoResult<()> {
        todo!("Implement reset password logic")
    }
}
