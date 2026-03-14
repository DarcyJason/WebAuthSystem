use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};

use crate::domain::auth::repository::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::common::time::ttl::TTL;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use crate::infrastructure::persistence::surrealdb::client::SurrealDBClient;

pub struct SurrealDBEmailVerificationTokenRepository {
    surrealdb_client: SurrealDBClient,
}

impl SurrealDBEmailVerificationTokenRepository {
    pub fn new(surrealdb_client: SurrealDBClient) -> Self {
        SurrealDBEmailVerificationTokenRepository { surrealdb_client }
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for SurrealDBEmailVerificationTokenRepository {
    async fn save(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        let ttl_duration = Duration::from_std(*ttl.value())
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenStoreUnavailable)?;
        let expires_at = Utc::now() + ttl_duration;
        let sql = r#"
            DELETE email_verification_token WHERE user_email = $user_email;
            CREATE email_verification_token SET
                user_email = $user_email,
                verification_token = $verification_token,
                expires_at = $expires_at;
        "#;
        self.surrealdb_client
            .client
            .query(sql)
            .bind(("user_email", user_email.to_owned()))
            .bind(("verification_token", mail_token))
            .bind(("expires_at", expires_at))
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenStoreUnavailable)?;
        Ok(())
    }

    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        let find_sql = r#"
            SELECT verification_token, expires_at FROM email_verification_token
            WHERE user_email = $user_email
            LIMIT 1;
        "#;
        let mut result = self
            .surrealdb_client
            .client
            .query(find_sql)
            .bind(("user_email", user_email.to_owned()))
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenNotFound)?;
        let record: Option<(VerificationToken, DateTime<Utc>)> = result
            .take(0)
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenNotFound)?;
        let (token, expires_at) = match record {
            Some(r) => r,
            None => return Ok(None),
        };

        if expires_at <= Utc::now() {
            let _ = self.delete(user_email).await;
            return Ok(None);
        }
        Ok(Some(token))
    }

    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        let delete_sql = r#"
            DELETE email_verification_token WHERE user_email = $user_email;
        "#;
        self.surrealdb_client
            .client
            .query(delete_sql)
            .bind(("user_email", user_email.to_owned()))
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenRemoveFailed)?;
        Ok(())
    }
}
