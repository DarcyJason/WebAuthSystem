use crate::application::auth::commands::send_verification_email_command::SendVerificationEmailCommand;
use crate::application::auth::results::send_verification_email_result::SendVerificationEmailResult;
use crate::application::errors::CaseResult;
use crate::domain::auth::repository::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::service::mail_service::MailService;
use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::infrastructure::errors::InfraError;
use crate::infrastructure::mail::verification_template::build_verification_email;
use std::sync::Arc;

pub struct SendVerificationEmailCase {
    auth_mail_service: Arc<dyn MailService>,
    auth_email_verification_cache: Arc<dyn EmailVerificationTokenRepository>,
}

impl SendVerificationEmailCase {
    pub fn new(
        auth_mail_service: Arc<dyn MailService>,
        auth_email_verification_cache: Arc<dyn EmailVerificationTokenRepository>,
    ) -> Self {
        SendVerificationEmailCase {
            auth_mail_service,
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        send_email_verification_command: SendVerificationEmailCommand,
    ) -> CaseResult<SendVerificationEmailResult> {
        let user_email = send_email_verification_command.email;
        let email_verification_token = VerificationToken::new();
        let expires_seconds = TTL::from_seconds(15 * 60);
        let verification_email = build_verification_email(
            user_email.clone(),
            email_verification_token.clone(),
            expires_seconds.clone(),
        );
        self.auth_email_verification_cache
            .save(&user_email, email_verification_token, expires_seconds)
            .await
            .map_err(InfraError::from)?;
        self.auth_mail_service
            .send_email(
                vec![user_email],
                MailSubject::new("Email Verification"),
                MailContent::new(verification_email),
            )
            .await
            .map_err(InfraError::from)?;
        Ok(SendVerificationEmailResult {})
    }
}
