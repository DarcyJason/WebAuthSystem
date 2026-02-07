use async_trait::async_trait;

use crate::{
    domain::auth::{
        entities::user::User,
        repositories::db::user_repo::{UserRepository, UserRepositoryError},
        value_objects::{user_email::UserEmail, user_id::UserId, user_name::UserName},
    },
    infrastructure::persistences::surrealdb::client::SurrealDBClient,
};

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
    async fn save_user(&self, user: User) -> Result<Option<User>, UserRepositoryError> {
        let users: Vec<User> = self
            .surrealdb_client
            .client
            .insert("user")
            .content(user)
            .await
            .map_err(|_| UserRepositoryError::InsertUserToSurrealDBFailed)?;
        Ok(users.get(0).cloned())
    }
    async fn find_user_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let existing_user: Option<User> = self
            .surrealdb_client
            .client
            .select(user_id.value().to_owned())
            .await
            .map_err(|_| UserRepositoryError::DeserializeRecordFailed)?;
        Ok(existing_user)
    }
    async fn find_user_by_name(
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
            .map_err(|_| UserRepositoryError::SurrealQLQueriedFailed)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializeRecordFailed)?;
        Ok(existing_user)
    }
    async fn find_user_by_email(
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
            .map_err(|_| UserRepositoryError::SurrealQLQueriedFailed)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializeRecordFailed)?;
        Ok(existing_user)
    }
    async fn find_user_by_name_or_email(
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
            .map_err(|_| UserRepositoryError::SurrealQLQueriedFailed)?;
        let existing_user: Option<User> = result
            .take(0)
            .map_err(|_| UserRepositoryError::DeserializeRecordFailed)?;
        Ok(existing_user)
    }
}
