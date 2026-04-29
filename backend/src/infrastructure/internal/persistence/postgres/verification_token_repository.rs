use crate::domain::auth::repositories::verification_token_repository::{
    VerificationTokenCommandRepository, VerificationTokenQueryRepository,
};
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_status::VerificationTokenStatus;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::error::{DomainResult, VerificationTokenRepositoryDbSnafu};
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::infrastructure::internal::persistence::postgres::client::PostgresClient;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresVerificationTokenRepository {
    pg_client: PostgresClient,
}

impl PostgresVerificationTokenRepository {
    pub fn new(pg_client: PostgresClient) -> Self {
        Self { pg_client }
    }
}

#[async_trait]
impl VerificationTokenCommandRepository for PostgresVerificationTokenRepository {
    async fn save(&self, token: &VerificationToken) -> DomainResult<VerificationToken> {
        sqlx::query(
            r#"
            INSERT INTO verification_tokens (id, user_id, value, kind, status, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(token.user_id().value())
        .bind(token.value().value())
        .bind(token.kind())
        .bind(token.status().value())
        .bind(token.created_at().value())
        .bind(token.expires_at().value())
        .execute(&self.pg_client.connection)
        .await
        .context(VerificationTokenRepositoryDbSnafu {
            message: format!(
                "save verification token for user:{} failed",
                token.user_id().value()
            ),
        })?;
        Ok(token.clone())
    }

    async fn mark_used(&self, value: &VerificationTokenValue) -> DomainResult<()> {
        sqlx::query(r#"UPDATE verification_tokens SET status = 'Used' WHERE value = $1"#)
            .bind(value.value())
            .execute(&self.pg_client.connection)
            .await
            .context(VerificationTokenRepositoryDbSnafu {
                message: "mark verification token used failed".to_string(),
            })?;
        Ok(())
    }

    async fn invalidate_by_user_id_and_kind(
        &self,
        user_id: &UserId,
        kind: VerificationTokenKind,
    ) -> DomainResult<()> {
        sqlx::query(
            r#"UPDATE verification_tokens
               SET status = 'Invalid'
               WHERE user_id = $1 AND kind = $2 AND status != 'Invalid'"#,
        )
        .bind(user_id.value())
        .bind(kind)
        .execute(&self.pg_client.connection)
        .await
        .context(VerificationTokenRepositoryDbSnafu {
            message: "invalidate verification tokens failed".to_string(),
        })?;
        Ok(())
    }
}

#[async_trait]
impl VerificationTokenQueryRepository for PostgresVerificationTokenRepository {
    async fn get_by_value(
        &self,
        value: &VerificationTokenValue,
    ) -> DomainResult<Option<VerificationToken>> {
        let row = sqlx::query(
            r#"SELECT value, user_id, kind::text, status::text, created_at, expires_at
               FROM verification_tokens WHERE value = $1"#,
        )
        .bind(value.value())
        .fetch_optional(&self.pg_client.connection)
        .await
        .context(VerificationTokenRepositoryDbSnafu {
            message: "get verification token by value failed".to_string(),
        })?;

        let token = match row {
            None => return Ok(None),
            Some(r) => {
                let raw_user_id: Uuid =
                    r.try_get("user_id")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read user_id failed".to_string(),
                        })?;
                let kind_str: String =
                    r.try_get("kind")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read kind failed".to_string(),
                        })?;
                let kind = match kind_str.as_str() {
                    "EmailVerification" => VerificationTokenKind::EmailVerification,
                    _ => VerificationTokenKind::PasswordReset,
                };
                let status_str: String =
                    r.try_get("status")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read status failed".to_string(),
                        })?;
                let status = match status_str.as_str() {
                    "Unused" => VerificationTokenStatus::Unused,
                    "Used" => VerificationTokenStatus::Used,
                    "Invalid" => VerificationTokenStatus::Invalid,
                    _ => VerificationTokenStatus::Unused,
                };
                let created_at: DateTime<Utc> =
                    r.try_get("created_at")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read created_at failed".to_string(),
                        })?;
                let expires_at: DateTime<Utc> =
                    r.try_get("expires_at")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read expires_at failed".to_string(),
                        })?;
                let token_value: String =
                    r.try_get("value")
                        .context(VerificationTokenRepositoryDbSnafu {
                            message: "read value failed".to_string(),
                        })?;
                let user_id = UserId::from_raw_user_id(raw_user_id.to_string())?;
                VerificationToken::from_parts(
                    VerificationTokenValue::from(token_value),
                    user_id,
                    kind,
                    status,
                    Timestamp::new(created_at),
                    Timestamp::new(expires_at),
                )
            }
        };
        Ok(Some(token))
    }
}
