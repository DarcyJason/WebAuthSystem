use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::auth::value_objects::tokens::refresh_token_id::RefreshTokenId;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::error::{DomainResult, RefreshTokenRepositoryDbSnafu};
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::infrastructure::internal::persistence::postgres::client::PostgresClient;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresRefreshTokenRepository {
    pg_client: PostgresClient,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pg_client: PostgresClient) -> Self {
        Self { pg_client }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn save(&self, refresh_token: &RefreshTokenEntity) -> DomainResult<RefreshTokenEntity> {
        sqlx::query(
            r#"INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, created_at)
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(refresh_token.id().value())
        .bind(refresh_token.user_id().value())
        .bind(refresh_token.token_hash().value())
        .bind(refresh_token.expires_at().value())
        .bind(refresh_token.created_at().value())
        .execute(&self.pg_client.connection)
        .await
        .context(RefreshTokenRepositoryDbSnafu {
            message: format!(
                "save refresh token for user:{} in postgres failed",
                refresh_token.user_id().value()
            ),
        })?;
        Ok(refresh_token.clone())
    }

    async fn get_by_hash(
        &self,
        hash: &RefreshTokenHash,
    ) -> DomainResult<Option<RefreshTokenEntity>> {
        let row = sqlx::query(
            r#"SELECT id, user_id, token_hash, expires_at, created_at
               FROM refresh_tokens WHERE token_hash = $1"#,
        )
        .bind(hash.value())
        .fetch_optional(&self.pg_client.connection)
        .await
        .context(RefreshTokenRepositoryDbSnafu {
            message: "get refresh token by hash failed".to_string(),
        })?;

        let entity = match row {
            None => return Ok(None),
            Some(r) => {
                let id: Uuid = r.try_get("id").context(RefreshTokenRepositoryDbSnafu {
                    message: "read id".to_string(),
                })?;
                let user_id: Uuid =
                    r.try_get("user_id")
                        .context(RefreshTokenRepositoryDbSnafu {
                            message: "read user_id".to_string(),
                        })?;
                let token_hash: String =
                    r.try_get("token_hash")
                        .context(RefreshTokenRepositoryDbSnafu {
                            message: "read token_hash".to_string(),
                        })?;
                let expires_at: DateTime<Utc> =
                    r.try_get("expires_at")
                        .context(RefreshTokenRepositoryDbSnafu {
                            message: "read expires_at".to_string(),
                        })?;
                let created_at: DateTime<Utc> =
                    r.try_get("created_at")
                        .context(RefreshTokenRepositoryDbSnafu {
                            message: "read created_at".to_string(),
                        })?;
                let user_id = UserId::from_raw_user_id(user_id.to_string())?;
                RefreshTokenEntity::from_parts(
                    RefreshTokenId::from(id),
                    user_id,
                    RefreshTokenHash::from_str(token_hash),
                    Timestamp::new(expires_at),
                    Timestamp::new(created_at),
                )
            }
        };
        Ok(Some(entity))
    }
}
