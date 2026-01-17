use crate::domain::user::entities::User;
use crate::domain::user::value_objects::{Email, HashPassword, Username};
use crate::{
    domain::error::DomainResult, domain::user::repositories::UserRepository,
    infrastructure::persistence::surreal::client::SurrealClient,
};
use async_trait::async_trait;
use surrealdb::RecordId;

#[derive(Debug, Clone)]
pub struct SurrealUserRepository {
    surreal: SurrealClient,
}

impl SurrealUserRepository {
    pub fn new(surreal: SurrealClient) -> Self {
        SurrealUserRepository { surreal }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>> {
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
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        let user: Option<User> = result
            .take(0)
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        Ok(user)
    }
    async fn find_by_id(&self, id: &RecordId) -> DomainResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE id = type::thing($id);
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("id", id.to_string()))
            .await
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        let user: Option<User> = result
            .take(0)
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        Ok(user)
    }
    async fn find_by_username(&self, username: &Username) -> DomainResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE username = $username;
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("username", username.to_string()))
            .await
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        let user: Option<User> = result
            .take(0)
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        Ok(user)
    }
    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE email = $email;
        "#;
        let mut result = self
            .surreal
            .client
            .query(sql)
            .bind(("email", email.to_string()))
            .await
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        let user: Option<User> = result
            .take(0)
            .map_err(|e| crate::domain::error::DomainError::Repository(e.to_string()))?;
        Ok(user)
    }
}
