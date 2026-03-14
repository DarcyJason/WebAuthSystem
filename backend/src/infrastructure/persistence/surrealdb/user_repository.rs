use async_trait::async_trait;

use crate::domain::auth::entity::user::User;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
use crate::infrastructure::persistence::surrealdb::client::SurrealDBClient;

pub struct SurrealDBUserRepository {
    surrealdb_client: SurrealDBClient,
}

impl SurrealDBUserRepository {
    pub fn new(surrealdb_client: SurrealDBClient) -> Self {
        SurrealDBUserRepository { surrealdb_client }
    }
}

#[async_trait]
impl UserRepository for SurrealDBUserRepository {
    async fn save(&self, user: User) -> Result<Option<User>, UserRepositoryError> {
        let users: Vec<User> = self
            .surrealdb_client
            .client
            .insert("user")
            .content(user)
            .await
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        Ok(users.first().cloned())
    }
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let existing_user: Option<User> = self
            .surrealdb_client
            .client
            .select(user_id.value().to_owned())
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        Ok(existing_user)
    }
    async fn find_by_name(
        &self,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        let find_sql = r#"
            SELECT * FROM user WHERE name = $user_name;
        "#;
        let mut result = self
            .surrealdb_client
            .client
            .query(find_sql)
            .bind(("user_name", user_name.to_owned()))
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializationFailed)?;
        Ok(existing_user)
    }
    async fn find_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let find_sql = r#"
            SELECT * FROM user WHERE email = $user_email;
        "#;
        let mut result = self
            .surrealdb_client
            .client
            .query(find_sql)
            .bind(("user_email", user_email.to_owned()))
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializationFailed)?;
        Ok(existing_user)
    }
    async fn find_by_name_or_email(
        &self,
        user_name: &UserName,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let find_sql = r#"
            SELECT * FROM user WHERE name = $user_name OR email = $user_email;
        "#;
        let mut result = self
            .surrealdb_client
            .client
            .query(find_sql)
            .bind(("user_name", user_name.to_owned()))
            .bind(("user_email", user_email.to_owned()))
            .await
            .map_err(|_| UserRepositoryError::StorageUnavailable)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializationFailed)?;
        Ok(existing_user)
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
        let updated_user: Option<User> = self
            .surrealdb_client
            .client
            .update(user.id().value().to_owned())
            .content(user)
            .await
            .map_err(|_| UserRepositoryError::PersistenceFailed)?;
        Ok(updated_user)
    }
}
