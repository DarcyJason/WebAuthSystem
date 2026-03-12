use std::sync::Arc;

use crate::application::{
    errors::{CaseError, CaseResult},
    queries::user::get_me_query::GetMeQuery,
    results::queries_results::user::get_me_result::GetMeResult,
};
use crate::domain::auth::services::access_token_service::AuthAccessTokenService;
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::errors::InfraError;

pub struct GetMeCase {
    user_repo: Arc<dyn UserRepository>,
    auth_access_token_service: Arc<dyn AuthAccessTokenService>,
}

impl GetMeCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_access_token_service: Arc<dyn AuthAccessTokenService>,
    ) -> Self {
        GetMeCase {
            user_repo,
            auth_access_token_service,
        }
    }
    pub async fn execute(&self, query: GetMeQuery) -> CaseResult<GetMeResult> {
        let access_claims = self
            .auth_access_token_service
            .decode_access_token(query.access_token)
            .map_err(InfraError::from)?;
        let user_id = access_claims.sub;
        let existing_user = self
            .user_repo
            .find_by_id(&user_id)
            .await
            .map_err(InfraError::from)?;
        let user = match existing_user {
            Some(user) => user,
            None => return Err(CaseError::UserNotFound),
        };
        Ok(GetMeResult {
            user_name: user.name().to_owned(),
            user_email: user.email().to_owned(),
            created_at: user.created_at().to_owned(),
        })
    }
}
