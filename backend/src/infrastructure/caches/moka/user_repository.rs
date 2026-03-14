use crate::domain::auth::entity::user::User;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
use async_trait::async_trait;

pub struct MokaUserRepository {
    pub moka_client: MokaClient<String, String>,
}

impl MokaUserRepository {
    pub fn new(moka_client: MokaClient<String, String>) -> Self {
        MokaUserRepository { moka_client }
    }

    async fn find_by_key(&self, key: &str) -> Result<Option<User>, UserRepositoryError> {
        let result = self.moka_client.client.get(key).await;
        User::from_redis_optional_value(result)
            .map_err(|_| UserRepositoryError::DeserializationFailed)
    }
}

#[async_trait]
impl UserRepository for MokaUserRepository {
    async fn save(&self, user: User) -> Result<Option<User>, UserRepositoryError> {
        let id_key = format!("user:id:{}", user.id().value());
        let email_key = format!("user:email:{}", user.email().value());
        let name_key = format!("user:name:{}", user.name().value());
        let payload = user
            .to_redis_value()
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        self.moka_client
            .client
            .insert(id_key.clone(), payload.clone())
            .await;
        self.moka_client
            .client
            .insert(email_key, payload.clone())
            .await;
        self.moka_client.client.insert(name_key, payload).await;
        let result = self.moka_client.client.get(&id_key).await;
        User::from_redis_optional_value(result)
            .map_err(|_| UserRepositoryError::DeserializationFailed)
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let key = format!("user:id:{}", user_id.value());
        self.find_by_key(&key).await
    }

    async fn find_by_name(
        &self,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        let key = format!("user:name:{}", user_name.value());
        self.find_by_key(&key).await
    }

    async fn find_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let key = format!("user:email:{}", user_email.value());
        self.find_by_key(&key).await
    }

    async fn find_by_name_or_email(
        &self,
        user_name: &UserName,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user = self.find_by_name(user_name).await?;
        if user.is_some() {
            return Ok(user);
        }
        self.find_by_email(user_email).await
    }

    async fn update_status_as_true(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let existing_user = self.find_by_email(user_email).await?;
        let mut user = match existing_user {
            Some(user) => user,
            None => return Ok(None),
        };
        user.mark_as_verified();
        self.save(user).await
    }
}
