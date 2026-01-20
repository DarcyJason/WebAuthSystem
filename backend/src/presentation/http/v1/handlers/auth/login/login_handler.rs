use std::sync::Arc;

use crate::{
    application::{
        commands::auth::login::LoginCommand, queries::auth::login::LoginResult,
        use_cases::auth::login_case::LoginCase,
    },
    infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository,
    presentation::http::v1::{
        errors::ApiResult, handlers::auth::login::payload::LoginPayload, response::ApiResponse,
        state::AppState,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::{info, instrument};

#[instrument(skip(app_state))]
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling login");
    let cmd = LoginCommand::try_from(payload)?;
    let auth_repo = SurrealAuthRepository::new(app_state.surreal.clone());
    let case = LoginCase::new(auth_repo);
    let data = case.execute(cmd).await?;
    let response = ApiResponse::<LoginResult>::ok(200, "Login success", data);
    info!("Finish handling login");
    Ok(response)
}
