use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    application::{
        commands::auth::register_command::RegisterCommand,
        use_cases::auth::register_case::RegisterCase,
    },
    presentation::http::v1::{
        errors::api_error::ApiResult,
        handlers::auth::register_handler::{
            request::RegisterRequestPayload, response::RegisterResponseData,
        },
        response::ApiResponse,
        states::app_state::AppState,
    },
};

pub async fn register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = RegisterCommand::try_from(payload)?;
    let case = RegisterCase::new(
        app_state.user_repo.clone(),
        app_state.auth_password_service.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = RegisterResponseData::from(result);
    let response =
        ApiResponse::<RegisterResponseData>::ok(200, "register successfully", response_data);
    Ok(response)
}
