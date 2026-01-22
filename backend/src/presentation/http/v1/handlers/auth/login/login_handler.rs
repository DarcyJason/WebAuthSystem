use std::sync::Arc;

use crate::presentation::http::v1::handlers::auth::login::request::LoginRequestPayload;
use crate::{
    application::{commands::auth::login::LoginCommand, use_cases::auth::login_case::LoginCase},
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult, handlers::auth::login::response::LoginResponseData,
        response::ApiResponse, state::AppState,
    },
};
use axum::{extract::State, response::IntoResponse, Json};
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
    let auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let case = LoginCase::new(auth_repo);
    let login_result = case.execute(cmd).await?;
    let login_response_data = LoginResponseData::from(login_result);
    let response = ApiResponse::<LoginResponseData>::ok(200, "Login success", login_response_data);
    info!("Finish handling login successfully");
    Ok(response)
}
