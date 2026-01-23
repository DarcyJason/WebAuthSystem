use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::errors::InfraResult;
use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;

#[derive(Debug, Clone)]
pub struct SurrealAuthRepository {
    surreal: SurrealClient,
}

impl SurrealAuthRepository {
    pub fn new(surreal: SurrealClient) -> Self {
        SurrealAuthRepository { surreal }
    }
    pub async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> InfraResult<Option<User>> {
        let user = self
            .surreal
            .user_repo()
            .find_by_username_or_email(&username, &email)
            .await?;
        if user.is_some() {
            return Err(SurrealDBError::RepositoryError("user already exists".to_string()).into());
        }
        let user = self
            .surreal
            .user_repo()
            .save(username, email, hash_password)
            .await?;
        Ok(user)
    }

    pub async fn login(&self, identity: LoginIdentity) -> InfraResult<Option<User>> {
        let user = match identity {
            LoginIdentity::Username(username) => {
                self.surreal.user_repo().find_by_username(&username).await?
            }
            LoginIdentity::Email(email) => self.surreal.user_repo().find_by_email(&email).await?,
        };
        if user.is_none() {
            return Err(SurrealDBError::RepositoryError("user not found".to_string()).into());
        }
        Ok(user)
    }

    pub async fn logout(&self, _user_id: &str) -> InfraResult<()> {
        todo!("Implement logout logic")
    }

    pub async fn forget_password(&self, _email: &str) -> InfraResult<()> {
        todo!("Implement forget password logic")
    }

    pub async fn reset_password(&self, _token: &str, _new_password: &str) -> InfraResult<()> {
        todo!("Implement reset password logic")
    }
}
