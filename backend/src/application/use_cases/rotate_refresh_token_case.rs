use crate::application::commands::rotate_refresh_token_command::RotateRefreshTokenCommand;
use crate::application::error::{
    ApplicationResult, DomainFailedSnafu, InvalidRefreshTokenSnafu, UserNotFoundSnafu,
};
use crate::application::results::rotate_refresh_token_result::RotateRefreshTokenResult;
use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::auth::services::access_token_service::AccessTokenService;
use crate::domain::auth::services::refresh_token_service::RefreshTokenService;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::repositories::user_repository::UserQueryRepository;
use crate::infrastructure::internal::layered::refresh_token_repository::LayeredRefreshTokenRepository;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::security::tokens::access_token::DefaultAccessTokenService;
use crate::infrastructure::internal::security::tokens::refresh_token::DefaultRefreshTokenService;
use chrono::{Duration, Utc};
use snafu::ResultExt;

pub struct RotateRefreshTokenCase {
    user_repo: LayeredUserRepository,
    access_token_service: DefaultAccessTokenService,
    refresh_token_service: DefaultRefreshTokenService,
    refresh_token_repo: LayeredRefreshTokenRepository,
}

impl RotateRefreshTokenCase {
    pub fn new(
        user_repo: LayeredUserRepository,
        access_token_service: DefaultAccessTokenService,
        refresh_token_service: DefaultRefreshTokenService,
        refresh_token_repo: LayeredRefreshTokenRepository,
    ) -> Self {
        Self {
            user_repo,
            access_token_service,
            refresh_token_service,
            refresh_token_repo,
        }
    }

    pub async fn execute(
        &self,
        cmd: RotateRefreshTokenCommand,
    ) -> ApplicationResult<RotateRefreshTokenResult> {
        let incoming = RefreshToken::new(cmd.refresh_token);
        let hash = RefreshTokenHash::from_refresh_token(&incoming);

        let stored = self
            .refresh_token_repo
            .get_by_hash(&hash)
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| InvalidRefreshTokenSnafu.build())?;

        if stored.expires_at().value() < &Utc::now() {
            return InvalidRefreshTokenSnafu.fail();
        }

        let user = self
            .user_repo
            .get_by_id(stored.user_id())
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| UserNotFoundSnafu.build())?;

        let access_token = self
            .access_token_service
            .generate(user.id(), user.access_token_version())
            .context(DomainFailedSnafu)?;

        let refresh_token = self.refresh_token_service.generate();
        let expires_at =
            Timestamp::new(Utc::now() + Duration::days(self.refresh_token_service.expires_in_days));
        let new_entity =
            RefreshTokenEntity::issue(user.id().to_owned(), &refresh_token, expires_at);
        self.refresh_token_repo
            .save(&new_entity)
            .await
            .context(DomainFailedSnafu)?;

        Ok(RotateRefreshTokenResult {
            access_token,
            refresh_token,
        })
    }
}
