use axum::Extension;
use axum::http::header::{AUTHORIZATION, SET_COOKIE};
use axum::{Json, http::HeaderValue, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use std::sync::Arc;

use crate::application::auth::cases::login_case::LoginCase;
use crate::application::auth::commands::login_command::LoginCommand;
use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::states::AppState;
use crate::presentation::http::v1::{
    handlers::auth::login_handler::{request::LoginRequestPayload, response::LoginResponseData},
    response::ApiResponse,
};

pub async fn login_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("Start handling login_handler");
    let cmd = LoginCommand::try_from(payload)?;
    let case = LoginCase::new(
        app_state.user_repo.clone(),
        app_state.access_token_service.clone(),
        app_state.refresh_token_service.clone(),
        app_state.password_service.clone(),
    );
    let result = case.execute(cmd).await?;
    let access_token = result.access_token.value();
    let refresh_token = result.refresh_token.value();
    let response_data = LoginResponseData::from(result.clone());
    let response = ApiResponse::<LoginResponseData>::ok(200, "Login successfully", response_data);
    let mut response = Json(response).into_response();
    response.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(
            &Cookie::build(("refresh_token", refresh_token))
                .http_only(true)
                .secure(true)
                .to_string(),
        )
        .unwrap(),
    );
    tracing::info!("Handle it successfully");
    Ok(response)
}
