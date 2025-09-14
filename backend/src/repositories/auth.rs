use crate::{
    database::client::DBClient,
    errors::app_error::{AppError, AppResult},
    models::{refresh_token::RefreshToken, user::User},
    utils::crypto::hash_password,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User>;
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>>;
    async fn find_user_by_id(&self, id: Thing) -> AppResult<Option<User>>;
    async fn store_refresh_token(
        &self,
        user_id: Thing,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()>;
    async fn find_refresh_token(
        &self,
        user_id: Thing,
        token_hash: String,
    ) -> AppResult<Option<RefreshToken>>;
    async fn delete_refresh_token(&self, token_id: Thing) -> AppResult<()>;
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
    async fn find_user_by_id(&self, id: Thing) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE id = $id
        "#;

        let mut result = self
            .surrealdb
            .query(sql)
            .bind(("id", id))
            .await
            .map_err(AppError::from)?;

        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
    async fn store_refresh_token(
        &self,
        user_id: Thing,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()> {
        let sql = "CREATE refresh_token SET user = $user, token_hash = $token_hash, expires_at = $expires_at;";
        self.surrealdb
            .query(sql)
            .bind(("user", user_id))
            .bind(("token_hash", token_hash))
            .bind(("expires_at", expires_at))
            .await?;
        Ok(())
    }
    async fn find_refresh_token(
        &self,
        user_id: Thing,
        token_hash: String,
    ) -> AppResult<Option<RefreshToken>> {
        let sql = "SELECT * FROM refresh_token WHERE user = $user AND token_hash = $token_hash AND expires_at > time::now() LIMIT 1;";
        let mut result = self
            .surrealdb
            .query(sql)
            .bind(("user", user_id))
            .bind(("token_hash", token_hash))
            .await?;
        let token: Option<RefreshToken> = result.take(0)?;
        Ok(token)
    }
    async fn delete_refresh_token(&self, token_id: Thing) -> AppResult<()> {
        let sql = "DELETE $id;";
        self.surrealdb.query(sql).bind(("id", token_id)).await?;
        Ok(())
    }
}
