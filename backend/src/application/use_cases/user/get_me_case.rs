use std::sync::Arc;

use crate::{
    application::{
        errors::{AppError, AppResult},
        queries::user::get_me_query::GetMeQuery,
        results::queries_results::user::get_me_result::GetMeResult,
    },
    domain::auth::{
        repositories::db::user_repo::UserRepository,
        services::token_service::AuthAccessTokenService,
    },
};

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
    pub async fn execute(&self, query: GetMeQuery) -> AppResult<GetMeResult> {
        let access_claims = self
            .auth_access_token_service
            .decode_access_token(query.access_token)
            .map_err(|_| AppError::DecodeAccessTokenFailed)?;
        let user_id = access_claims.sub;
        let existing_user = self
            .user_repo
            .find_user_by_id(&user_id)
            .await
            .map_err(|_| AppError::SurrealDBError)?;
        let user = match existing_user {
            Some(user) => user,
            None => return Err(AppError::UserNotFound),
        };
        Ok(GetMeResult {
            user_name: user.name().to_owned(),
            user_email: user.email().to_owned(),
            created_at: user.created_at().to_owned(),
        })
    }
}
