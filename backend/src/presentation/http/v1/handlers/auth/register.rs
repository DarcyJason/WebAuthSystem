use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use crate::{
    application::{commands::auth::register::RegisterCommand, handlers::auth::register_handler::RegisterHandler},
    infrastructure::persistence::surreal::user_repository::SurrealUserRepository,
    presentation::http::v1::{response::ApiResponse, result::AppResult, state::AppState},
};

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(cmd): Json<RegisterCommand>,
) -> AppResult<impl IntoResponse> {
    let user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let handler = RegisterHandler::new(user_repo);
    let data = handler.handle(cmd).await?;
    let response = ApiResponse::<()>::ok(200, "Register success", data);
    Ok(response)
}
