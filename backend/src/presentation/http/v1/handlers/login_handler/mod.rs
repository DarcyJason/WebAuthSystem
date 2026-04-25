pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::commands::login_command::LoginCommand;
use crate::application::use_cases::login_case::LoginCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::login_handler::request::LoginRequestPayload;
use crate::presentation::http::v1::handlers::login_handler::response::LoginResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::http::header::{AUTHORIZATION, SET_COOKIE};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequestPayload,
    responses(
        (status = 200, description = "Login successful — access token in Authorization header, refresh token in Set-Cookie", body = LoginResponseData),
        (status = 400, description = "Invalid credentials or email not verified"),
        (status = 404, description = "User not found"),
    )
)]
#[instrument(skip(app_state, payload), fields(name_or_email=%payload.name_or_email))]
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling login request");
    let cmd: LoginCommand = payload.try_into()?;
    let case = LoginCase::new(
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
        app_state.access_token_service.clone(),
        app_state.refresh_token_service.clone(),
        app_state.refresh_token_repo.clone(),
    );
    let result = case.execute(cmd).await?;
    let access_token = result.access_token.value();
    let refresh_token = result.refresh_token.value();
    let cookie = Cookie::build(("refresh_token", refresh_token.to_string()))
        .path("/")
        .max_age(time::Duration::days(
            app_state.refresh_token_service.expires_in_days,
        ))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build();
    let mut response_header = HeaderMap::new();
    response_header.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    response_header.append(SET_COOKIE, cookie.to_string().parse().unwrap());
    let response_data = LoginResponseData::from(&result);
    let response = ApiResponse::<LoginResponseData>::ok(
        Some(response_header),
        "Login successfully",
        response_data,
    );
    tracing::info!("handling login request successfully");
    Ok(response)
}
