use async_trait::async_trait;

use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use crate::{
    domain::{
        auth::value_objects::verification_token::VerificationToken, common::time::ttl::TTL,
        user::value_objects::user_email::UserEmail,
    },
    infrastructure::caches::redis::client::RedisClient,
};

pub struct RedisEmailVerificationTokenRepository {
    redis_client: RedisClient,
}

impl RedisEmailVerificationTokenRepository {
    pub fn new(redis_client: RedisClient) -> Self {
        RedisEmailVerificationTokenRepository { redis_client }
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for RedisEmailVerificationTokenRepository {
    async fn save(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let result: () = redis::cmd("SET")
            .arg(format!("email_verify:{}", user_email.value()))
            .arg(mail_token.value())
            .arg("EX")
            .arg(ttl.value().as_secs())
            .query_async(&mut conn)
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenStoreUnavailable)?;
        Ok(result)
    }
    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let key = format!("email_verify:{}", user_email.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenNotFound)?;
        let result = result.map(VerificationToken::from);
        Ok(result)
    }
    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        let mut conn = self.redis_client.client.clone();
        let result: () = redis::cmd("DEL")
            .arg(format!("email_verify:{}", user_email.value()))
            .query_async(&mut conn)
            .await
            .map_err(|_| EmailVerificationTokenRepositoryError::TokenRemoveFailed)?;
        Ok(result)
    }
}
