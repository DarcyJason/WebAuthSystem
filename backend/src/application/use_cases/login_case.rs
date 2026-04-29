use crate::application::commands::login_command::LoginCommand;
use crate::application::error::{
    AccountIsBannedSnafu, ApplicationResult, DomainFailedSnafu, EmailNotVerifiedSnafu,
    InvalidCredentialsSnafu, UserNotFoundSnafu,
};
use crate::application::results::login_result::LoginResult;
use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::repositories::refresh_token_repository::RefreshTokenCommandRepository;
use crate::domain::auth::services::access_token_service::AccessTokenService;
use crate::domain::auth::services::password_service::PasswordService;
use crate::domain::auth::services::refresh_token_service::RefreshTokenService;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::repositories::user_repository::UserQueryRepository;
use crate::domain::user::value_objects::user::user_name_or_user_email::UserNameOrUserEmail;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use crate::infrastructure::internal::layered::refresh_token_repository::LayeredRefreshTokenRepository;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::security::password::Argon2PasswordService;
use crate::infrastructure::internal::security::tokens::access_token::DefaultAccessTokenService;
use crate::infrastructure::internal::security::tokens::refresh_token::DefaultRefreshTokenService;
use chrono::{Duration, Utc};
use snafu::ResultExt;

pub struct LoginCase {
    user_repo: LayeredUserRepository,
    password_service: Argon2PasswordService,
    access_token_service: DefaultAccessTokenService,
    refresh_token_service: DefaultRefreshTokenService,
    refresh_token_repo: LayeredRefreshTokenRepository,
}

impl LoginCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        password_service: Argon2PasswordService,
        access_token_service: DefaultAccessTokenService,
        refresh_token_service: DefaultRefreshTokenService,
        refresh_token_repo: LayeredRefreshTokenRepository,
    ) -> Self {
        LoginCase {
            user_repo,
            password_service,
            access_token_service,
            refresh_token_service,
            refresh_token_repo,
        }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> ApplicationResult<LoginResult> {
        let (name, email) = match cmd.name_or_email {
            UserNameOrUserEmail::UserName(n) => (Some(n), None),
            UserNameOrUserEmail::UserEmail(e) => (None, Some(e)),
        };
        let existing_user = self
            .user_repo
            .get_by_name_or_email(&name, &email)
            .await
            .context(DomainFailedSnafu)?;
        let user = match existing_user {
            Some(user) => user,
            None => return UserNotFoundSnafu.fail(),
        };
        let password_credential = user
            .credentials()
            .iter()
            .find_map(|c| c.kind().password_credential())
            .cloned()
            .ok_or_else(|| UserNotFoundSnafu.build())?;
        let matched = self
            .password_service
            .verify_password(password_credential, cmd.plain_password)
            .context(DomainFailedSnafu)?;
        if !matched {
            return InvalidCredentialsSnafu.fail();
        }
        if user.status() == &UserStatus::Banned {
            return AccountIsBannedSnafu.fail();
        }
        if user.status() == &UserStatus::EmailNotVerified {
            return EmailNotVerifiedSnafu.fail();
        }
        let access_token = self
            .access_token_service
            .generate(user.id(), user.access_token_version())
            .context(DomainFailedSnafu)?;
        let refresh_token = self.refresh_token_service.generate();
        let expires_at =
            Timestamp::new(Utc::now() + Duration::days(self.refresh_token_service.expires_in_days));
        let refresh_token_entity =
            RefreshTokenEntity::issue(user.id().to_owned(), &refresh_token, expires_at);
        self.refresh_token_repo
            .save(&refresh_token_entity)
            .await
            .context(DomainFailedSnafu)?;
        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }
}
