use crate::application::commands::register_command::RegisterCommand;
use crate::application::error::{
    ApplicationResult, DomainFailedSnafu, PasswordServiceFailedSnafu, UserAlreadyExistsSnafu,
};
use crate::application::results::register_result::RegisterResult;
use crate::domain::auth::repositories::verification_token_repository::VerificationTokenCommandRepository;
use crate::domain::auth::services::mail_service::MailService;
use crate::domain::auth::services::password_service::PasswordService;
use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::user::aggregates::user::UserEntity;
use crate::domain::user::entities::credential::Credential;
use crate::domain::user::repositories::user_repository::{
    UserCommandRepository, UserQueryRepository,
};
use crate::domain::user::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::user::value_objects::credential::credential_status::CredentialStatus;
use crate::infrastructure::external::resend::ResendMailService;
use crate::infrastructure::external::resend::verification_email_template::build_verification_email;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use crate::infrastructure::internal::security::password::Argon2PasswordService;
use crate::infrastructure::internal::security::tokens::verification_token::DefaultVerificationTokenService;
use snafu::ResultExt;

pub struct RegisterCase {
    user_repo: LayeredUserRepository,
    password_service: Argon2PasswordService,
    verification_token_repo: LayeredVerificationTokenRepository,
    verification_token_service: DefaultVerificationTokenService,
    mail_service: ResendMailService,
}

impl RegisterCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        password_service: Argon2PasswordService,
        verification_token_repo: LayeredVerificationTokenRepository,
        verification_token_service: DefaultVerificationTokenService,
        mail_service: ResendMailService,
    ) -> Self {
        RegisterCase {
            user_repo,
            password_service,
            verification_token_repo,
            verification_token_service,
            mail_service,
        }
    }

    pub async fn execute(&self, cmd: RegisterCommand) -> ApplicationResult<RegisterResult> {
        let existing_user = self
            .user_repo
            .get_by_name_or_email(&Some(cmd.name.clone()), &Some(cmd.email.clone()))
            .await
            .context(DomainFailedSnafu)?;
        if existing_user.is_some() {
            return UserAlreadyExistsSnafu.fail();
        }

        let password_credential = self
            .password_service
            .hash_password(cmd.plain_password)
            .context(PasswordServiceFailedSnafu)?;
        let credentials = Credential::new(
            &CredentialKind::new(&password_credential),
            &CredentialStatus::Active,
        );
        let user = UserEntity::new(cmd.name, cmd.email.clone(), vec![credentials]);
        let created_user = self
            .user_repo
            .save(&user)
            .await
            .context(DomainFailedSnafu)?;

        // Issue email verification token
        let token = self
            .verification_token_service
            .issue_email_verification(created_user.id().to_owned());
        let expires_secs = self
            .verification_token_service
            .email_verify_expires_in_seconds;
        let saved_token = self
            .verification_token_repo
            .save(&token)
            .await
            .context(DomainFailedSnafu)?;

        // Send verification email (best-effort: don't fail registration if email fails)
        let ttl = TTL::from_seconds(expires_secs as u64);
        let html = build_verification_email(cmd.email.clone(), saved_token, ttl);
        let mail = Mail::new(
            MailSubject::new("Verify your email address"),
            MailContent::new(html),
        );
        if let Err(e) = self.mail_service.send_email(vec![cmd.email], mail).await {
            tracing::warn!("failed to send verification email: {}", e);
        }

        Ok(RegisterResult {
            user_id: created_user.id().to_owned(),
        })
    }
}
