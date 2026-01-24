use std::sync::Arc;

use crate::domain::auth::repositories::db::SurrealAuthRepositoryAdapter;
use crate::domain::auth::repositories::token::AuthTokenRepositoryAdapter;
use crate::infrastructure::token::token_repositoy::TokenRepository;
use crate::presentation::http::v1::handlers::auth::login::request::LoginRequestPayload;
use crate::{
    application::{commands::auth::login::LoginCommand, use_cases::auth::login_case::LoginCase},
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult, handlers::auth::login::response::LoginResponseData,
        response::ApiResponse, state::AppState,
    },
};
use axum::http::HeaderValue;
use axum::http::header::{AUTHORIZATION, SET_COOKIE};
use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(post, path = "/api/v1/auth/login", request_body = LoginRequestPayload, responses(
    (status = 200, description = "login success", body = LoginResponseData)
), tag = "Auth")]
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling login successfully");
    let cmd = LoginCommand::try_from(payload)?;
    let surreal_auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let surreal_auth_repo_adapter = SurrealAuthRepositoryAdapter::new(surreal_auth_repo);
    let token_repo = TokenRepository::new(&app_state.app_config.jwt.secret.clone());
    let auth_token_repo_adapter = AuthTokenRepositoryAdapter::new(Arc::new(token_repo));
    let case = LoginCase::new(surreal_auth_repo_adapter, auth_token_repo_adapter);
    let login_result = case.execute(cmd).await?;
    let login_response_data = LoginResponseData::from(login_result.clone());
    let response = ApiResponse::<LoginResponseData>::ok(200, "Login success", login_response_data);
    let mut response = Json(response).into_response();
    response.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", login_result.access_token.as_str())).unwrap(),
    );
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(
            &Cookie::build(("refresh_token", login_result.refresh_token.as_str()))
                .http_only(true)
                .secure(true)
                .to_string(),
        )
        .unwrap(),
    );
    info!("Finish handling login successfully");
    Ok(response)
}
