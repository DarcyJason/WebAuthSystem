use async_trait::async_trait;

use crate::{
    domain::{
        auth::{
            repositories::cache::email_verification_cache::{
                EmailVerificationCache, EmailVerificationCacheError,
            },
            value_objects::{ user_email::UserEmail},
        },
        common::time::ttl::TTL,
    },
    infrastructure::caches::redis::client::RedisClient,
};
use crate::domain::auth::value_objects::verification_token::VerificationToken;

pub struct RedisEmailVerificationCache {
    redis_client: RedisClient,
}

impl RedisEmailVerificationCache {
    pub fn new(redis_client: RedisClient) -> Self {
        RedisEmailVerificationCache { redis_client }
    }
}

#[async_trait]
impl EmailVerificationCache for RedisEmailVerificationCache {
    async fn save_email_verification_token(
        &mut self,
        user_email: UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationCacheError> {
        let result: () = redis::cmd("SET")
            .arg(format!("email_verify:{}", user_email.value()))
            .arg(mail_token.value())
            .arg("EX")
            .arg(ttl.value().as_secs())
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerificationCacheError::TokenStoreUnavailable)?;
        Ok(result)
    }
    async fn get_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationCacheError> {
        let key = format!("email_verify:{}", user_email.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerificationCacheError::TokenNotFound)?;
        let result = match result {
            Some(token) => Some(VerificationToken::from(token)),
            None => None,
        };
        Ok(result)
    }
    async fn delete_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<(), EmailVerificationCacheError> {
        let result: () = redis::cmd("DEL")
            .arg(format!("email_verify:{}", user_email.value()))
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerificationCacheError::TokenRemoveFailed)?;
        Ok(result)
    }
}
