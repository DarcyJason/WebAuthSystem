use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::errors::InfraResult;
use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use surrealdb::RecordId;

#[derive(Debug, Clone)]
pub struct SurrealUserRepository {
    surreal: SurrealClient,
}

impl SurrealUserRepository {
    pub fn new(surreal: SurrealClient) -> Self {
        SurrealUserRepository { surreal }
    }
    pub async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> InfraResult<Option<User>> {
        let sql = r#"
            CREATE user CONTENT {
                id: rand::uuid::v4(),
                username: $username,
                email: $email,
                hash_password: $hash_password,
                created_at: time::now(),
                updated_at: time::now(),
            };
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("username", username.to_string()))
            .bind(("email", email.to_string()))
            .bind(("hash_password", hash_password.to_string()))
            .await
            .map_err(|_| SurrealDBError::ExecuteQueryError)?;
        let user: Option<User> = result
            .take(0)
            .map_err(|_| SurrealDBError::ParseRecordToUserError)?;
        Ok(user)
    }
    pub async fn find_by_id(&self, id: &RecordId) -> InfraResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE id = type::thing($id);
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("id", id.to_string()))
            .await
            .map_err(|_| SurrealDBError::ExecuteQueryError)?;
        let user: Option<User> = result
            .take(0)
            .map_err(|_| SurrealDBError::ParseRecordToUserError)?;
        Ok(user)
    }
    pub async fn find_by_username(&self, username: &Username) -> InfraResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE username = $username;
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("username", username.to_string()))
            .await
            .map_err(|_| SurrealDBError::ExecuteQueryError)?;
        let user: Option<User> = result
            .take(0)
            .map_err(|_| SurrealDBError::ParseRecordToUserError)?;
        Ok(user)
    }
    pub async fn find_by_email(&self, email: &Email) -> InfraResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE email = $email;
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("email", email.to_string()))
            .await
            .map_err(|_| SurrealDBError::ExecuteQueryError)?;
        let user: Option<User> = result
            .take(0)
            .map_err(|_| SurrealDBError::ParseRecordToUserError)?;
        Ok(user)
    }
    pub async fn find_by_username_or_email(
        &self,
        username: &Username,
        email: &Email,
    ) -> InfraResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE username = $username OR email = $email;
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("username", username.to_string()))
            .bind(("email", email.to_string()))
            .await
            .map_err(|_| SurrealDBError::ExecuteQueryError)?;
        let user: Option<User> = result
            .take(0)
            .map_err(|_| SurrealDBError::ParseRecordToUserError)?;
        Ok(user)
    }
}
