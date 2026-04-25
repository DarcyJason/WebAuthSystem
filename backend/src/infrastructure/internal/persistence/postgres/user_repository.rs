use crate::domain::error::{DomainResult, UserRepositoryDbSnafu};
use crate::domain::user::aggregates::user::UserEntity;
use crate::domain::user::entities::credential::Credential;
use crate::domain::user::repositories::user_repository::{
    UserCommandRepository, UserQueryRepository,
};
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::user::value_objects::credential::credential_status::CredentialStatus;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::domain::user::value_objects::user::user_name::UserName;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use crate::infrastructure::internal::persistence::postgres::client::PostgresClient;
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::types::Json;

#[derive(Debug, Clone)]
pub struct PostgresUserRepository {
    pg_client: PostgresClient,
}

impl PostgresUserRepository {
    pub fn new(pg_client: PostgresClient) -> Self {
        PostgresUserRepository { pg_client }
    }
}

#[async_trait]
impl UserCommandRepository for PostgresUserRepository {
    async fn save(&self, user: &UserEntity) -> DomainResult<UserEntity> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
                INSERT INTO users (id, name, email, credentials, status, access_token_version, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, name, email, credentials, status, access_token_version, created_at, updated_at
            "#,
        )
        .bind(user.id())
        .bind(user.name())
        .bind(user.email())
        .bind(Json(user.credentials()))
        .bind(user.status())
        .bind(user.access_token_version())
        .bind(user.created_at())
        .bind(user.updated_at())
        .fetch_one(&self.pg_client.connection)
        .await
        .context(UserRepositoryDbSnafu {
            message: format!("create user:{} in postgres failed", user.id().value()),
        })?;
        Ok(user)
    }
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<UserEntity> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
                UPDATE users SET status = $1 WHERE id = $2 RETURNING id, name, email, credentials, status, access_token_version, created_at, updated_at
            "#
        )
            .bind(user_status)
            .bind(user_id)
            .fetch_one(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("update status on user:{} in postgres failed", user_id.value())
                }
            )?;
        Ok(user)
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<UserEntity> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
                UPDATE users SET credentials = $1 WHERE id = $2 RETURNING id, name, email, credentials, status, access_token_version, created_at, updated_at
            "#
        )
            .bind(Json(vec![Credential::new(&CredentialKind::new(user_password_credential), &CredentialStatus::Active)]))
            .bind(user_id)
            .fetch_one(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("update password_credential on user:{} in postgres failed", user_id.value())
                }
            )?;
        Ok(user)
    }
    async fn update_access_token_version(
        &self,
        user_id: &UserId,
        access_token_version: &AccessTokenVersion,
    ) -> DomainResult<UserEntity> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
                UPDATE users SET access_token_version = $1 WHERE id = $2 RETURNING id, name, email, credentials, status, access_token_version, created_at, updated_at
            "#,
        )
        .bind(access_token_version)
        .bind(user_id)
        .fetch_one(&self.pg_client.connection)
        .await
        .context(UserRepositoryDbSnafu {
            message: format!(
                "update access_token_version on user:{} in postgres failed",
                user_id.value()
            ),
        })?;
        Ok(user)
    }
}

#[async_trait]
impl UserQueryRepository for PostgresUserRepository {
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<UserEntity>> {
        let optional_user = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT id, name, email, credentials, status, access_token_version, created_at, updated_at FROM users where id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("get user by id: {} in postgres failed", user_id.value())
                }
            )?;
        Ok(optional_user)
    }
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<UserEntity>> {
        let optional_user = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT id, name, email, credentials, status, access_token_version, created_at, updated_at FROM users WHERE name = $1
            "#
        )
            .bind(user_name)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("get user by name: {} in postgres failed", user_name.value())
                }
            )?;
        Ok(optional_user)
    }
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<UserEntity>> {
        let optional_user = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT id, name, email, credentials, status, access_token_version, created_at, updated_at FROM users WHERE email = $1
            "#
        )
            .bind(user_email)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("get user by email: {} in postgres failed", user_email.value())
                }
            )?;
        Ok(optional_user)
    }
    async fn get_by_name_or_email(
        &self,
        user_name: &Option<UserName>,
        user_email: &Option<UserEmail>,
    ) -> DomainResult<Option<UserEntity>> {
        let optional_user = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT id, name, email, credentials, status, access_token_version, created_at, updated_at FROM users WHERE name = $1 OR email = $2
            "#
        )
            .bind(user_name)
            .bind(user_email)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                match (user_name, user_email) {
                    (Some(user_name), None) => UserRepositoryDbSnafu {
                        message: format!("get user by name or email: {} in postgres failed", user_name.value())
                    },
                    (None, Some(user_email)) => UserRepositoryDbSnafu {
                        message: format!("get user by name or email: {} in postgres failed", user_email.value())
                    },
                    _ => UserRepositoryDbSnafu {
                        message: "get user by name or email: user name and user email are not provided".to_string()
                    }
                }
            )?;
        Ok(optional_user)
    }
}
