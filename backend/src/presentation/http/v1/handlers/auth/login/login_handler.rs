use std::sync::Arc;

use crate::{
    application::{commands::auth::login::LoginCommand, use_cases::auth::login_case::LoginCase},
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult,
        handlers::auth::login::{request::LoginRequest, response::LoginResponse},
        response::ApiResponse,
        state::AppState,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(post, path = "/api/v1/auth/login", request_body = LoginRequest, responses(
    (status = 200, description = "login success", body = LoginResponse)
), tag = "Auth")]
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling login");
    let cmd = LoginCommand::try_from(request)?;
    let auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let case = LoginCase::new(auth_repo);
    let login_result = case.execute(cmd).await?;
    let login_response = LoginResponse::from(login_result);
    let response = ApiResponse::<LoginResponse>::ok(200, "Login success", login_response);
    info!("Finish handling login");
    Ok(response)
}
