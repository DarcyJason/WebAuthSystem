use crate::application::use_cases::user::get_me_case::GetMeCase;
use crate::domain::auth::repositories::token::AuthTokenRepositoryAdapter;
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::user::repositories::db::SurrealUserRepositoryAdapter;
use crate::infrastructure::persistence::surreal::user_repository::SurrealUserRepository;
use crate::infrastructure::token::token_repositoy::TokenRepository;
use crate::presentation::http::v1::errors::{ApiError, ApiResult};
use crate::presentation::http::v1::handlers::user::get_me::response::GetMeResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use crate::presentation::http::v1::state::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(post, path = "/api/v1/user/me", responses(
    (status = 200, description = "get me successfully", body = GetMeResponseData)
), tag = "User")]
pub async fn get_me_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling get_me");
    let authorization = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;
    let raw_access_token = match authorization.strip_prefix("Bearer ") {
        Some(raw_token) => raw_token,
        None => return Err(ApiError::Unauthorized),
    };
    let access_token = AccessToken::new(raw_access_token.to_string());
    let surreal_user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let surreal_user_repo_adapter = SurrealUserRepositoryAdapter::new(surreal_user_repo);
    let token_repo = TokenRepository::new(&app_state.app_config.jwt.secret.clone());
    let auth_token_repo_adapter = AuthTokenRepositoryAdapter::new(Arc::new(token_repo));
    let case = GetMeCase::new(surreal_user_repo_adapter, auth_token_repo_adapter);
    let get_me_result = case.execute(access_token).await?;
    let get_me_response_data = GetMeResponseData::from(get_me_result);
    let response =
        ApiResponse::<GetMeResponseData>::ok(200, "get me successfully", get_me_response_data);
    info!("Finish handling get_me");
    Ok(response)
}
