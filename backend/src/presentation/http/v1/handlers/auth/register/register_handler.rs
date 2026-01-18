use std::sync::Arc;

use crate::{
    application::{
        commands::auth::register::RegisterCommand, use_cases::auth::register_case::RegisterCase,
    },
    infrastructure::persistence::surreal::{
        auth_repository::SurrealAuthRepository, user_repository::SurrealUserRepository,
    },
    presentation::http::v1::{errors::ApiResult, response::ApiResponse, state::AppState},
};
use axum::{Json, extract::State, response::IntoResponse};
use tracing::instrument;

#[instrument]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(cmd): Json<RegisterCommand>,
) -> ApiResult<impl IntoResponse> {
    let user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let auth_repo = SurrealAuthRepository::new(user_repo);
    let case = RegisterCase::new(auth_repo);
    let (message, data) = case.execute(cmd).await?;
    let response = ApiResponse::<()>::ok(200, message, data);
    Ok(response)
}
