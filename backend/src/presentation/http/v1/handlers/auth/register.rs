use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use crate::{
    application::{commands::auth::register::RegisterCommand, use_cases::auth::register_case::RegisterCase},
    infrastructure::persistence::surreal::{auth_repository::SurrealAuthRepository, user_repository::SurrealUserRepository},
    presentation::http::v1::{response::ApiResponse, result::AppResult, state::AppState},
};

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(cmd): Json<RegisterCommand>,
) -> AppResult<impl IntoResponse> {
    let user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let auth_repo = SurrealAuthRepository::new(user_repo);
    let case = RegisterCase::new(auth_repo);
    let data = case.execute(cmd).await?;
    let response = ApiResponse::<()>::ok(200, "Register success", data);
    Ok(response)
}
