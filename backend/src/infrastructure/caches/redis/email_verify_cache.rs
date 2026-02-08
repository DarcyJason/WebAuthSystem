use async_trait::async_trait;

use crate::{
    domain::{
        auth::{
            repositories::cache::email_verify_cache::{EmailVerifyCache, EmailVerifyCacheError},
            value_objects::{mail_token::MailToken, user_email::UserEmail},
        },
        common::time::ttl::TTL,
    },
    infrastructure::caches::redis::client::RedisClient,
};

pub struct RedisEmailVerifyCache {
    redis_client: RedisClient,
}

impl RedisEmailVerifyCache {
    pub fn new(redis_client: RedisClient) -> Self {
        RedisEmailVerifyCache { redis_client }
    }
}

#[async_trait]
impl EmailVerifyCache for RedisEmailVerifyCache {
    async fn save_verification_token(
        &mut self,
        user_email: UserEmail,
        mail_token: MailToken,
        ttl: TTL,
    ) -> Result<(), EmailVerifyCacheError> {
        let result: () = redis::cmd("SET")
            .arg(format!("email_verify:{}", user_email.value()))
            .arg(mail_token.value())
            .arg("EX")
            .arg(ttl.value().as_secs())
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerifyCacheError::TokenStoreUnavailable)?;
        Ok(result)
    }
    async fn get_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<Option<MailToken>, EmailVerifyCacheError> {
        let key = format!("email_verify:{}", user_email.value());
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerifyCacheError::TokenNotFound)?;
        let result = match result {
            Some(token) => Some(MailToken::from(token)),
            None => None,
        };
        Ok(result)
    }
    async fn delete_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<(), EmailVerifyCacheError> {
        let result: () = redis::cmd("DEL")
            .arg(format!("email_verify:{}", user_email.value()))
            .query_async(&mut self.redis_client.client)
            .await
            .map_err(|_| EmailVerifyCacheError::TokenRemoveFailed)?;
        Ok(result)
    }
}
