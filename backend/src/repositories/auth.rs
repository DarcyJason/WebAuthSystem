use crate::{
    database::client::DBClient,
    errors::app_error::{AppError, AppResult},
    models::user::User,
    utils::crypto::hash_password,
};
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User>;
    // async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    // async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
}

#[async_trait]
impl AuthRepository for DBClient {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User> {
        let hashed_password = hash_password(password)?;
        let sql = r#"
            CREATE user CONTENT {
                id: rand::uuid::v4(),
                name: $name,
                email: $email,
                password: $password,
                created_at: time::now(),
                updated_at: time::now()
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
}
