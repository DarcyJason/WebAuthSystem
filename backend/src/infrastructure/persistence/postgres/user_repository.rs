use crate::domain::error::{DomainResult, UserRepositoryDbSnafu};
use crate::domain::identities::aggregates::user::User;
use crate::domain::identities::entities::credential::Credential;
use crate::domain::identities::repositories::user_repository::UserRepository;
use crate::domain::identities::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::identities::value_objects::credential::credential_status::CredentialStatus;
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::user::user_name::UserName;
use crate::domain::identities::value_objects::user::user_status::UserStatus;
use crate::infrastructure::persistence::postgres::client::PostgresClient;
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::types::Json;

pub struct PostgresUserRepository {
    pg_client: PostgresClient,
}

impl PostgresUserRepository {
    pub fn new(pg_client: PostgresClient) -> Self {
        PostgresUserRepository { pg_client }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> DomainResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id, name, email, credentials, status, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, name, email, credentials, status, created_at, updated_at
            "#,
        )
        .bind(user.id())
        .bind(user.name())
        .bind(user.email())
        .bind(Json(user.credentials()))
        .bind(user.status())
        .bind(user.created_at())
        .bind(user.updated_at())
        .fetch_one(&self.pg_client.connection)
        .await
        .context(UserRepositoryDbSnafu {
            message: format!("create user:{} in postgres", user.id().value()),
        })?;
        Ok(user)
    }
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<User>> {
        let optional_user = sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, email, credentials, status, created_at, updated_at FROM users where id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("get user by id: {} in postgres", user_id.value())
                }
            )?;
        Ok(optional_user)
    }
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<User>> {
        let optional_user = sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, email, credentials, status, created_at, updated_at FROM users WHERE name = $1
            "#
        )
            .bind(user_name)
            .fetch_optional(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("get user by name: {} in postgres", user_name.value())
                }
            )?;
        Ok(optional_user)
    }
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<User>> {
        let optional_user = sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, email, credentials, status, created_at, updated_at FROM users WHERE email = $1
            "#
        )
        .bind(user_email)
        .fetch_optional(&self.pg_client.connection)
        .await
        .context(
            UserRepositoryDbSnafu {
                message: format!("get user by email: {} in postgres", user_email.value())
            }
        )?;
        Ok(optional_user)
    }
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
                UPDATE users SET status = $1 WHERE id = $2 RETURNING id, name, email, credentials, status, created_at, updated_at
            "#
        )
            .bind(user_status)
            .bind(user_id)
            .fetch_one(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("update status on user:{} in postgres", user_id.value())
                }
            )?;
        Ok(user)
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
                UPDATE users SET credentials = $1 WHERE id = $2 RETURNING id, name, email, credentials, status, created_at, updated_at
            "#
        )
            .bind(Json(vec![Credential::new(&CredentialKind::new(user_password_credential), &CredentialStatus::Active)]))
            .bind(user_id)
            .fetch_one(&self.pg_client.connection)
            .await
            .context(
                UserRepositoryDbSnafu {
                    message: format!("update password_credential on user:{} in postgres", user_id.value())
                }
            )?;
        Ok(user)
    }
}
