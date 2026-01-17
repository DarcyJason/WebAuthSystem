use std::sync::Arc;

use crate::{
    application::{
        commands::auth::login::LoginCommand, queries::auth::login::LoginView,
        use_cases::auth::login_case::LoginCase,
    },
    infrastructure::persistence::surreal::{
        auth_repository::SurrealAuthRepository, user_repository::SurrealUserRepository,
    },
    presentation::http::v1::{errors::ApiResult, response::ApiResponse, state::AppState},
};
use axum::{Json, extract::State, response::IntoResponse};

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(cmd): Json<LoginCommand>,
) -> ApiResult<impl IntoResponse> {
    let user_repo = SurrealUserRepository::new(app_state.surreal.clone());
    let auth_repo = SurrealAuthRepository::new(user_repo);
    let case = LoginCase::new(auth_repo);
    let (message, data) = case.execute(cmd).await?;
    let response = ApiResponse::<LoginView>::ok(200, message, data);
    Ok(response)
}
