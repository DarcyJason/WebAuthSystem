use crate::application::commands::change_password_command::ChangePasswordCommand;
use crate::application::error::{
    ApplicationResult, DomainFailedSnafu, InvalidCredentialsSnafu, UserNotFoundSnafu,
};
use crate::application::results::change_password_result::ChangePasswordResult;
use crate::domain::auth::services::password_service::PasswordService;
use crate::domain::user::repositories::user_repository::{
    UserCommandRepository, UserQueryRepository,
};
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::security::password::Argon2PasswordService;
use snafu::ResultExt;

pub struct ChangePasswordCase {
    user_id: UserId,
    user_repo: LayeredUserRepository,
    password_service: Argon2PasswordService,
}

impl ChangePasswordCase {
    pub fn new(
        user_id: UserId,
        user_repo: LayeredUserRepository,
        password_service: Argon2PasswordService,
    ) -> Self {
        Self {
            user_id,
            user_repo,
            password_service,
        }
    }

    pub async fn execute(
        &self,
        cmd: ChangePasswordCommand,
    ) -> ApplicationResult<ChangePasswordResult> {
        let user = self
            .user_repo
            .get_by_id(&self.user_id)
            .await
            .context(DomainFailedSnafu)?
            .ok_or_else(|| UserNotFoundSnafu.build())?;

        let password_credential = user
            .credentials()
            .iter()
            .find_map(|c| c.kind().password_credential())
            .cloned()
            .ok_or_else(|| UserNotFoundSnafu.build())?;

        let matched = self
            .password_service
            .verify_password(password_credential, cmd.current_password)
            .context(DomainFailedSnafu)?;
        if !matched {
            return InvalidCredentialsSnafu.fail();
        }

        let new_credential = self
            .password_service
            .hash_password(cmd.new_password)
            .context(DomainFailedSnafu)?;

        self.user_repo
            .update_password_credential(&self.user_id, &new_credential)
            .await
            .context(DomainFailedSnafu)?;

        let new_access_token_version = AccessTokenVersion::new();
        self.user_repo
            .update_access_token_version(&self.user_id, &new_access_token_version)
            .await
            .context(DomainFailedSnafu)?;

        Ok(ChangePasswordResult)
    }
}
