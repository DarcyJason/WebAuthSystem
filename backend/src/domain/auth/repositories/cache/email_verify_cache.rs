use async_trait::async_trait;

use crate::domain::{
    auth::value_objects::{mail_token::MailToken, user_email::UserEmail},
    common::time::ttl::TTL,
};

pub enum EmailVerifyCacheError {
    TokenStoreUnavailable,
    TokenNotFound,
    TokenRemoveFailed,
}

#[async_trait]
pub trait EmailVerifyCache: Send + Sync {
    async fn save_verification_token(
        &mut self,
        user_email: UserEmail,
        mail_token: MailToken,
        ttl: TTL,
    ) -> Result<(), EmailVerifyCacheError>;
    async fn get_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<Option<MailToken>, EmailVerifyCacheError>;
    async fn delete_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<(), EmailVerifyCacheError>;
}
