use crate::application::commands::reset_password_command::ResetPasswordCommand;
use crate::application::error::{
    ApplicationResult, DomainFailedSnafu, PasswordServiceFailedSnafu,
    VerificationTokenAlreadyUsedSnafu, VerificationTokenExpiredSnafu,
    VerificationTokenNotFoundSnafu,
};
use crate::application::results::reset_password_result::ResetPasswordResult;
use crate::domain::auth::repositories::verification_token_repository::VerificationTokenRepository;
use crate::domain::auth::services::password_service::PasswordService;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_status::VerificationTokenStatus;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::user::repositories::user_repository::UserCommandRepository;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use crate::infrastructure::internal::security::password::Argon2PasswordService;
use chrono::Utc;
use snafu::ResultExt;

pub struct ResetPasswordCase {
    user_repo: LayeredUserRepository,
    password_service: Argon2PasswordService,
    verification_token_repo: LayeredVerificationTokenRepository,
}

impl ResetPasswordCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        password_service: Argon2PasswordService,
        verification_token_repo: LayeredVerificationTokenRepository,
    ) -> Self {
        Self {
            user_repo,
            password_service,
            verification_token_repo,
        }
    }

    pub async fn execute(
        &self,
        cmd: ResetPasswordCommand,
    ) -> ApplicationResult<ResetPasswordResult> {
        let token_value = VerificationTokenValue::from(cmd.token);
        let token = self
            .verification_token_repo
            .get_by_value(&token_value)
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| VerificationTokenNotFoundSnafu.build())?;

        if token.expires_at().value() < &Utc::now() {
            return VerificationTokenExpiredSnafu.fail();
        }
        if token.status().value() == &VerificationTokenStatus::Used {
            return VerificationTokenAlreadyUsedSnafu.fail();
        }
        // Ensure this token was issued for password reset, not email verification
        if !matches!(token.kind(), VerificationTokenKind::PasswordReset) {
            return VerificationTokenNotFoundSnafu.fail();
        }

        let new_credential = self
            .password_service
            .hash_password(cmd.new_password)
            .context(PasswordServiceFailedSnafu)?;

        self.user_repo
            .update_password_credential(&token.user_id(), &new_credential)
            .await
            .context(DomainFailedSnafu)?;

        self.verification_token_repo
            .mark_used(&token_value)
            .await
            .context(DomainFailedSnafu)?;

        Ok(ResetPasswordResult)
    }
}
