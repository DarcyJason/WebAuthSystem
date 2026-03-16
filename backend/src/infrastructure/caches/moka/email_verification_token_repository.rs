use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::auth::repository::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::common::time::ttl::TTL;
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;

pub struct MokaEmailVerificationTokenRepository {
    pub moka_client: MokaClient<String, String>,
}

impl MokaEmailVerificationTokenRepository {
    pub fn new(moka_client: MokaClient<String, String>) -> Self {
        MokaEmailVerificationTokenRepository { moka_client }
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for MokaEmailVerificationTokenRepository {
    async fn save(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        let key = format!("email_verify:{}", user_email.value());
        let payload = (
            mail_token.value().to_owned(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or(0)
                .saturating_add(ttl.value().as_secs()),
        );
        let payload = serde_json::to_string(&payload)
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenStoreUnavailable)?;
        self.moka_client.client.insert(key, payload).await;
        Ok(())
    }

    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        let key = format!("email_verify:{}", user_email.value());
        let payload = self.moka_client.client.get(&key).await;
        let payload = match payload {
            Some(payload) => payload,
            None => return Ok(None),
        };
        let payload: (String, u64) = serde_json::from_str(payload.as_str())
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenNotFound)?;
        let now_unix_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        if payload.1 <= now_unix_seconds {
            self.moka_client.client.invalidate(&key).await;
            return Ok(None);
        }
        Ok(Some(VerificationToken::from(payload.0)))
    }

    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        self.moka_client
            .client
            .invalidate(&format!("email_verify:{}", user_email.value()))
            .await;
        Ok(())
    }
}
