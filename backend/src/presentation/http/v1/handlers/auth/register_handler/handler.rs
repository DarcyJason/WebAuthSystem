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
    let register_command = RegisterCommand::try_from(payload)?;
    let case = RegisterCase::new(
        app_state.user_repo.clone(),
        app_state.auth_password_service.clone(),
    );
    let register_result = case.execute(register_command).await?;
    let response_data = RegisterResponseData::from(register_result);
    let response =
        ApiResponse::<RegisterResponseData>::ok(200, "register successfully", response_data);
    Ok(response)
}
