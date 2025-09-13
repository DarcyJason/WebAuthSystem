use crate::{
    database::client::DBClient,
    errors::app_error::{AppError, AppResult},
    models::user::User,
    utils::crypto::hash_password,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User>;
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>>;
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
}

#[async_trait]
impl AuthRepository for DBClient {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User> {
        let hashed_password = hash_password(password)?;
        let sql = r#"
            CREATE user CONTENT {
                name: $name,
                email: $email,
                password: $password
            }
        "#;
        let mut result = self
            .surrealdb
            .query(sql)
            .bind(("name", name))
            .bind(("email", email))
            .bind(("password", hashed_password))
            .await
            .map_err(AppError::from)?;
        let created: Option<User> = result.take(0)?;
        created.ok_or_else(|| AppError::UserCreationError)
    }
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE email = $email LIMIT 1
        "#;
        let mut result = self
            .surrealdb
            .query(sql)
            .bind(("email", email))
            .await
            .map_err(AppError::from)?;
        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE id = $id
        "#;

        let mut result = self
            .surrealdb
            .query(sql)
            .bind(("id", id.to_string()))
            .await
            .map_err(AppError::from)?;

        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
}
