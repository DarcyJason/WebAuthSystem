use axum::{Extension, Json, response::IntoResponse};
use std::sync::Arc;

use crate::application::auth::cases::register_case::RegisterCase;
use crate::application::auth::commands::register_command::RegisterCommand;
use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::states::AppState;
use crate::presentation::http::v1::{
    handlers::auth::register_handler::{
        request::RegisterRequestPayload, response::RegisterResponseData,
    },
    response::ApiResponse,
};

pub async fn register_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = RegisterCommand::try_from(payload)?;
    let case = RegisterCase::new(
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = RegisterResponseData::from(result);
    let response =
        ApiResponse::<RegisterResponseData>::ok(200, "register successfully", response_data);
    Ok(response)
}
