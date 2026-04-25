use crate::application::commands::verify_command::VerifyCommand;
use crate::application::error::{
    ApplicationResult, DomainFailedSnafu, UserNotFoundSnafu, VerificationTokenAlreadyUsedSnafu,
    VerificationTokenExpiredSnafu, VerificationTokenNotFoundSnafu,
};
use crate::application::results::verify_result::VerifyResult;
use crate::domain::auth::repositories::verification_token_repository::VerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::user::repositories::user_repository::{
    UserCommandRepository, UserQueryRepository,
};
use crate::domain::user::value_objects::user::user_status::UserStatus;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use chrono::Utc;
use snafu::ResultExt;

pub struct VerifyCase {
    verification_token_repo: LayeredVerificationTokenRepository,
    user_repo: LayeredUserRepository,
}

impl VerifyCase {
    pub fn new(
        verification_token_repo: LayeredVerificationTokenRepository,
        user_repo: LayeredUserRepository,
    ) -> Self {
        Self {
            verification_token_repo,
            user_repo,
        }
    }

    pub async fn execute(&self, cmd: VerifyCommand) -> ApplicationResult<VerifyResult> {
        let token = self
            .verification_token_repo
            .get_by_value(&cmd.token_value)
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| VerificationTokenNotFoundSnafu.build())?;

        if token.expires_at().value() < &Utc::now() {
            return VerificationTokenExpiredSnafu.fail();
        }
        if token.used().value() {
            return VerificationTokenAlreadyUsedSnafu.fail();
        }
        // Only EmailVerification tokens activate the account
        if let VerificationTokenKind::EmailVerification = token.kind() {
            let user = self
                .user_repo
                .get_by_id(&token.user_id())
                .await
                .context(DomainFailedSnafu)?
                .ok_or_else(|| UserNotFoundSnafu.build())?;

            if user.status() == &UserStatus::EmailNotVerified {
                self.user_repo
                    .update_status(&token.user_id(), &UserStatus::Active)
                    .await
                    .context(DomainFailedSnafu)?;
            }
        }
        self.verification_token_repo
            .mark_used(&cmd.token_value)
            .await
            .context(DomainFailedSnafu)?;
        Ok(VerifyResult {})
    }
}
