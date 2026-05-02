use crate::application::commands::forgot_password_command::ForgotPasswordCommand;
use crate::application::error::{ApplicationResult, DomainFailedSnafu};
use crate::application::results::forgot_password_result::ForgotPasswordResult;
use crate::domain::auth::repositories::verification_token_repository::VerificationTokenCommandRepository;
use crate::domain::auth::services::mail_service::MailService;
use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::user::repositories::user_repository::UserQueryRepository;
use crate::infrastructure::external::resend::ResendMailService;
use crate::infrastructure::external::resend::verification_email_template::build_password_reset_email;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use crate::infrastructure::internal::security::tokens::verification_token::DefaultVerificationTokenService;
use snafu::ResultExt;

pub struct ForgotPasswordCase {
    user_repo: LayeredUserRepository,
    verification_token_repo: LayeredVerificationTokenRepository,
    verification_token_service: DefaultVerificationTokenService,
    mail_service: ResendMailService,
}

impl ForgotPasswordCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        verification_token_repo: LayeredVerificationTokenRepository,
        verification_token_service: DefaultVerificationTokenService,
        mail_service: ResendMailService,
    ) -> Self {
        Self {
            user_repo,
            verification_token_repo,
            verification_token_service,
            mail_service,
        }
    }

    pub async fn execute(
        &self,
        cmd: ForgotPasswordCommand,
    ) -> ApplicationResult<ForgotPasswordResult> {
        if let Some(user) = self
            .user_repo
            .get_by_email(&cmd.email)
            .await
            .context(DomainFailedSnafu)?
        {
            let token = self
                .verification_token_service
                .issue_password_reset(user.id().to_owned());
            let expires_secs = self
                .verification_token_service
                .password_reset_expires_in_seconds;
            let saved_token = self
                .verification_token_repo
                .save(&token)
                .await
                .context(DomainFailedSnafu)?;

            let ttl = TTL::from_seconds(expires_secs as u64);
            let html = build_password_reset_email(cmd.email.clone(), saved_token, ttl);
            let mail = Mail::new(
                MailSubject::new("Reset your password"),
                MailContent::new(html),
            );
            if let Err(e) = self.mail_service.send_email(vec![cmd.email], mail).await {
                tracing::warn!("failed to send password reset email: {}", e);
            }
        }
        Ok(ForgotPasswordResult)
    }
}
