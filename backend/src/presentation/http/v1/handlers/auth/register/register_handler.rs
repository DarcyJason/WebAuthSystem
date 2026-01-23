use std::sync::Arc;

use crate::domain::auth::repositories::SurrealAuthRepositoryAdapter;
use crate::{
    application::{
        commands::auth::register::RegisterCommand, use_cases::auth::register_case::RegisterCase,
    },
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult,
        handlers::auth::register::{
            request::RegisterRequestPayload, response::RegisterResponseData,
        },
        response::ApiResponse,
        state::AppState,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(post, path = "/api/v1/auth/register", request_body = RegisterRequestPayload, responses(
    (status = 200, description = "register success", body = RegisterResponseData)
),tag = "Auth")]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling register successfully");
    let cmd = RegisterCommand::try_from(payload)?;
    let surreal_auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let surreal_auth_repo_adapter = SurrealAuthRepositoryAdapter::new(surreal_auth_repo);
    let case = RegisterCase::new(surreal_auth_repo_adapter);
    let register_result = case.execute(cmd).await?;
    let register_response_data = RegisterResponseData::from(register_result);
    let response =
        ApiResponse::<RegisterResponseData>::ok(200, "register success", register_response_data);
    info!("Finish handling register successfully");
    Ok(response)
}
