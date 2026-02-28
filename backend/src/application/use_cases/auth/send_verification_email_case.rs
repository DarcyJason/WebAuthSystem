use crate::application::commands::auth::send_verification_email_command::SendVerificationEmailCommand;
use crate::application::errors::{AppError, AppResult};
use crate::application::results::commands_results::auth::send_verification_email_result::SendVerificationEmailResult;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::auth::value_objects::mail_content::MailContent;
use crate::domain::auth::value_objects::mail_subject::MailSubject;
use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::domain::user::value_objects::user_email::UserEmail;
use crate::domain::{
    auth::repositories::cache::email_verification_cache::EmailVerificationCache,
    common::time::ttl::TTL,
};
use crate::infrastructure::mail::verification_template::build_verification_email;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SendVerificationEmailCase {
    auth_mail_service: Arc<dyn AuthMailService>,
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
}

impl SendVerificationEmailCase {
    pub fn new(
        auth_mail_service: Arc<dyn AuthMailService>,
        auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
    ) -> Self {
        SendVerificationEmailCase {
            auth_mail_service,
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        send_email_verification_command: SendVerificationEmailCommand,
    ) -> AppResult<SendVerificationEmailResult> {
        let user_email = send_email_verification_command.email;
        let email_verification_token = VerificationToken::new();
        let expires_seconds = TTL::from_seonds(15 * 60);
        let verification_email = build_verification_email(
            user_email.clone(),
            email_verification_token.clone(),
            expires_seconds.clone(),
        );
        self.auth_email_verification_cache
            .lock()
            .await
            .save_email_verification_token(
                user_email.clone(),
                email_verification_token,
                expires_seconds,
            )
            .await
            .map_err(|_| AppError::SaveEmailVerificationTokenFailed)?;
        self.auth_mail_service
            .send_email(
                UserEmail::new("notnone@email.homeryland.com")
                    .map_err(|_| AppError::SystemOwnerEmailInvalid)?,
                vec![user_email],
                MailSubject::new("Email Verification"),
                MailContent::new(verification_email),
            )
            .await
            .map_err(|_| AppError::SendEmailVerificationFailed)?;
        Ok(SendVerificationEmailResult {})
    }
}
