use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::domain::user::entities::user::user_email::UserEmail;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;

pub struct LayeredEmailVerificationTokenRepository {
    l1_cache: Arc<dyn EmailVerificationTokenRepository>,
    l2_cache: Arc<dyn EmailVerificationTokenRepository>,
    source_repo: Arc<dyn EmailVerificationTokenRepository>,
}

impl LayeredEmailVerificationTokenRepository {
    pub fn new(
        l1_cache: Arc<dyn EmailVerificationTokenRepository>,
        l2_cache: Arc<dyn EmailVerificationTokenRepository>,
        source_repo: Arc<dyn EmailVerificationTokenRepository>,
    ) -> Self {
        LayeredEmailVerificationTokenRepository {
            l1_cache,
            l2_cache,
            source_repo,
        }
    }

    async fn warm_up_l1(&self, user_email: &UserEmail, mail_token: VerificationToken, ttl: TTL) {
        let _ = self.l1_cache.save(user_email, mail_token, ttl).await;
    }

    async fn warm_up_l2_and_l1(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) {
        let _ = self
            .l2_cache
            .save(user_email, mail_token.clone(), ttl.clone())
            .await;
        self.warm_up_l1(user_email, mail_token, ttl).await;
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for LayeredEmailVerificationTokenRepository {
    async fn save(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        self.source_repo
            .save(user_email, mail_token.clone(), ttl.clone())
            .await?;
        self.warm_up_l2_and_l1(user_email, mail_token, ttl).await;
        Ok(())
    }

    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        if let Ok(Some(mail_token)) = self.l1_cache.get_by_user_email(user_email).await {
            return Ok(Some(mail_token));
        }
        if let Ok(Some(mail_token)) = self.l2_cache.get_by_user_email(user_email).await {
            return Ok(Some(mail_token));
        }
        self.source_repo.get_by_user_email(user_email).await
    }

    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        self.source_repo.delete(user_email).await?;
        let _ = self.l2_cache.delete(user_email).await;
        let _ = self.l1_cache.delete(user_email).await;
        Ok(())
    }
}
