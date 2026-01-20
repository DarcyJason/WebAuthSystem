use std::sync::Arc;

use crate::{
    application::{
        commands::auth::register::RegisterCommand, use_cases::auth::register_case::RegisterCase,
    },
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult,
        handlers::auth::register::{request::RegisterRequest, response::RegisterResponse},
        response::ApiResponse,
        state::AppState,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(post, path = "/api/v1/auth/register", request_body = RegisterRequest, responses(
    (status = 200, description = "register success", body = RegisterResponse)
),tag = "Auth")]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling register");
    let cmd = RegisterCommand::try_from(request)?;
    let auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let case = RegisterCase::new(auth_repo);
    let register_result = case.execute(cmd).await?;
    let register_response = RegisterResponse::from(register_result);
    let response = ApiResponse::<RegisterResponse>::ok(200, "register success", register_response);
    info!("Finish handling register");
    Ok(response)
}
