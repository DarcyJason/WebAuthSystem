use ntex::web::{
    Responder,
    types::{Json, State},
};

use crate::{
    dtos::{api_response::ApiResponse, request::register::RegisterRequest},
    errors::app_error::AppResult,
    repositories::auth::AuthRepository,
    state::AppState,
};

pub async fn register_handler(
    app_state: State<AppState>,
    payload: Json<RegisterRequest>,
) -> AppResult<impl Responder> {
    let payload = payload.into_inner();
    let user = app_state
        .db_client
        .create_user(payload.name, payload.email, payload.password)
        .await?;
    Ok(ApiResponse::success("Register success", user))
}
