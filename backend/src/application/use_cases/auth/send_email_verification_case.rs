use crate::application::errors::{AppError, AppResult};
use crate::application::results::commands_results::auth::send_email_verification_result::SendEmailVerificationResult;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::auth::value_objects::mail_content::MailContent;
use crate::domain::auth::value_objects::mail_subject::MailSubject;
use crate::domain::auth::value_objects::user_email::UserEmail;
use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::{
    application::commands::auth::send_email_verification_command::SendEmailVerificationCommand,
    domain::{
        auth::repositories::cache::email_verification_cache::EmailVerificationCache,
        common::time::ttl::TTL,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SendEmailVerificationCase {
    auth_mail_service: Arc<dyn AuthMailService>,
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
}

impl SendEmailVerificationCase {
    pub fn new(
        auth_mail_service: Arc<dyn AuthMailService>,
        auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
    ) -> Self {
        SendEmailVerificationCase {
            auth_mail_service,
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        send_email_verification_command: SendEmailVerificationCommand,
    ) -> AppResult<SendEmailVerificationResult> {
        let user_email = send_email_verification_command.email;
        let email_verification_token = VerificationToken::new();
        self.auth_email_verification_cache
            .lock()
            .await
            .save_email_verification_token(
                user_email.clone(),
                email_verification_token,
                TTL::from_seonds(15 * 60),
            )
            .await
            .map_err(|_| AppError::SaveEmailVerificationTokenFailed)?;
        self.auth_mail_service
            .send_email(
                UserEmail::new("notnone@homeryland.com")
                    .map_err(|_| AppError::SystemOwnerEmailInvalid)?,
                vec![user_email],
                MailSubject::new("Email Verification"),
                MailContent::new("mail_content"),
            )
            .await
            .map_err(|_| AppError::SendEmailVerificationFailed)?;
        Ok(SendEmailVerificationResult {})
    }
}
