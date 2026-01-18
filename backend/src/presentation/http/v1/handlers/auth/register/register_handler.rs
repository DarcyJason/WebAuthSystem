use std::sync::Arc;

use crate::{
    application::{
        commands::auth::register::RegisterCommand, queries::auth::register::RegisterResult,
        use_cases::auth::register_case::RegisterCase,
    },
    infrastructure::persistence::surreal::{
        auth_repository::SurrealAuthRepository, user_repository::SurrealUserRepository,
    },
    presentation::http::v1::{
        errors::ApiResult, handlers::auth::register::payload::RegisterPaylaod,
        response::ApiResponse, state::AppState,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::{info, instrument};

#[instrument(skip(app_state))]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPaylaod>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling register");
    let cmd = RegisterCommand::try_from(payload)?;
    let user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let auth_repo = SurrealAuthRepository::new(user_repo);
    let case = RegisterCase::new(auth_repo);
    let (message, data) = case.execute(cmd).await?;
    let response = ApiResponse::<RegisterResult>::ok(200, message, data);
    info!("Finish handling register");
    Ok(response)
}
