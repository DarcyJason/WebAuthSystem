use crate::domain::auth::entity::user::User;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
use async_trait::async_trait;

pub struct RedisUserRepository {
    redis_client: RedisClient,
}

impl RedisUserRepository {
    pub fn new(redis_client: RedisClient) -> Self {
        RedisUserRepository { redis_client }
    }
}

#[async_trait]
impl UserRepository for RedisUserRepository {
    async fn save(&self, user: User) -> Result<Option<User>, UserRepositoryError> {
        let id_key = format!("user:id:{}", user.id().value());
        let email_key = format!("user:email:{}", user.email().value());
        let name_key = format!("user:name:{}", user.name().value());
        let payload =
            serde_json::to_string(&user).map_err(|_| UserRepositoryError::PersistenceFailed)?;
        let mut conn = self.redis_client.client.clone();
        redis::cmd("SET")
            .arg(&id_key)
            .arg(&payload)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        redis::cmd("SET")
            .arg(&email_key)
            .arg(&payload)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        redis::cmd("SET")
            .arg(&name_key)
            .arg(&payload)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        let result: Option<String> = redis::cmd("GET")
            .arg(&id_key)
            .query_async(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        match result {
            Some(user) => {
                return Ok(Some(
                    serde_json::from_str::<User>(&user)
                        .map_err(|_| UserRepositoryError::DeserializationFailed)?,
                ));
            }
            None => return Ok(None),
        };
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let id_key = format!("user:id:{}", user_id.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(id_key)
            .query_async(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        match result {
            Some(user) => {
                return Ok(Some(
                    serde_json::from_str::<User>(&user)
                        .map_err(|_| UserRepositoryError::DeserializationFailed)?,
                ));
            }
            None => return Ok(None),
        };
    }

    async fn find_by_name(
        &self,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let name_key = format!("user:name:{}", user_name.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(name_key)
            .query_async(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        match result {
            Some(user) => {
                return Ok(Some(
                    serde_json::from_str::<User>(&user)
                        .map_err(|_| UserRepositoryError::DeserializationFailed)?,
                ));
            }
            None => return Ok(None),
        };
    }

    async fn find_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let email_key = format!("user:email:{}", user_email.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(email_key)
            .query_async(&mut conn)
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        match result {
            Some(user) => {
                return Ok(Some(
                    serde_json::from_str::<User>(&user)
                        .map_err(|_| UserRepositoryError::DeserializationFailed)?,
                ));
            }
            None => return Ok(None),
        };
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
