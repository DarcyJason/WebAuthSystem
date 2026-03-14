use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::auth::repository::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::common::time::ttl::TTL;
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct EmailVerificationTokenCacheValue {
    token: String,
    expires_at_unix_seconds: u64,
}

pub struct MokaEmailVerificationTokenRepository {
    pub moka_client: MokaClient<String, String>,
}

impl MokaEmailVerificationTokenRepository {
    pub fn new(moka_client: MokaClient<String, String>) -> Self {
        MokaEmailVerificationTokenRepository { moka_client }
    }
}

impl MokaEmailVerificationTokenRepository {
    fn key_for(user_email: &UserEmail) -> String {
        format!("email_verify:{}", user_email.value())
    }

    fn now_unix_seconds() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0)
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
        let key = Self::key_for(user_email);
        let payload = EmailVerificationTokenCacheValue {
            token: mail_token.value().to_owned(),
            expires_at_unix_seconds: Self::now_unix_seconds().saturating_add(ttl.value().as_secs()),
        };
        let payload = serde_json::to_string(&payload)
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenStoreUnavailable)?;
        self.moka_client.client.insert(key, payload).await;
        Ok(())
    }

    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        let key = Self::key_for(user_email);
        let payload = self.moka_client.client.get(&key).await;
        let payload = match payload {
            Some(payload) => payload,
            None => return Ok(None),
        };
        let payload: EmailVerificationTokenCacheValue = serde_json::from_str(payload.as_str())
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenNotFound)?;
        if payload.expires_at_unix_seconds <= Self::now_unix_seconds() {
            self.moka_client.client.invalidate(&key).await;
            return Ok(None);
        }
        Ok(Some(VerificationToken::from(payload.token)))
    }

    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        self.moka_client
            .client
            .invalidate(&Self::key_for(user_email))
            .await;
        Ok(())
    }
}
