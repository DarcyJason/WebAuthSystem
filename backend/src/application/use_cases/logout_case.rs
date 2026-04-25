use crate::application::error::{ApplicationResult, DomainFailedSnafu};
use crate::domain::user::repositories::user_repository::UserCommandRepository;
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use snafu::ResultExt;

pub struct LogoutCase {
    user_repo: LayeredUserRepository,
}

impl LogoutCase {
    pub fn new(user_repo: LayeredUserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: &UserId) -> ApplicationResult<()> {
        let new_access_token_version = AccessTokenVersion::new();
        self.user_repo
            .update_access_token_version(user_id, &new_access_token_version)
            .await
            .context(DomainFailedSnafu)?;
        Ok(())
    }
}
