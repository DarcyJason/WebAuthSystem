use crate::application::commands::resend_verification_command::ResendVerificationCommand;
use crate::application::error::{ApplicationResult, DomainFailedSnafu, UserNotFoundSnafu};
use crate::domain::auth::repositories::verification_token_repository::VerificationTokenRepository;
use crate::domain::auth::services::mail_service::MailService;
use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::user::repositories::user_repository::UserQueryRepository;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use crate::infrastructure::external::resend::ResendMailService;
use crate::infrastructure::external::resend::verification_email_template::build_verification_email;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use crate::infrastructure::internal::security::tokens::verification_token::DefaultVerificationTokenService;
use snafu::ResultExt;

pub struct ResendVerificationCase {
    user_repo: LayeredUserRepository,
    verification_token_repo: LayeredVerificationTokenRepository,
    verification_token_service: DefaultVerificationTokenService,
    mail_service: ResendMailService,
}

impl ResendVerificationCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        verification_token_repo: LayeredVerificationTokenRepository,
        verification_token_service: DefaultVerificationTokenService,
        mail_service: ResendMailService,
    ) -> Self {
        ResendVerificationCase {
            user_repo,
            verification_token_repo,
            verification_token_service,
            mail_service,
        }
    }

    pub async fn execute(&self, cmd: ResendVerificationCommand) -> ApplicationResult<()> {
        let user = self
            .user_repo
            .get_by_name_or_email(&None, &Some(cmd.email.clone()))
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| UserNotFoundSnafu.build())?;

        // If user is already verified, we could either return success or an error.
        // Returning success is often better to avoid leaking user info or causing confusion.
        // But for internal logic, let's check it.
        if user.status() != &UserStatus::EmailNotVerified {
            return Ok(());
        }

        // Issue new email verification token
        let token = self
            .verification_token_service
            .issue_email_verification(user.id().to_owned());
        let expires_secs = self
            .verification_token_service
            .email_verify_expires_in_seconds;
        let saved_token = self
            .verification_token_repo
            .save(&token)
            .await
            .context(DomainFailedSnafu)?;

        // Send verification email
        let ttl = TTL::from_seconds(expires_secs as u64);
        let html = build_verification_email(cmd.email.clone(), saved_token, ttl);
        let mail = Mail::new(
            MailSubject::new("Verify your email address"),
            MailContent::new(html),
        );
        
        self.mail_service
            .send_email(vec![cmd.email], mail)
            .await
            .context(DomainFailedSnafu)?;

        Ok(())
    }
}
