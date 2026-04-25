pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::commands::rotate_refresh_token_command::RotateRefreshTokenCommand;
use crate::application::use_cases::rotate_refresh_token_case::RotateRefreshTokenCase;
use crate::presentation::http::v1::error::{ApiError, ApiResult};
use crate::presentation::http::v1::handlers::rotate_refresh_token_handler::response::RotateRefreshTokenResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::extract::State;
use axum::http::header::{AUTHORIZATION, SET_COOKIE};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/rotate-refresh-token",
    tag = "Auth",
    responses(
        (status = 200, description = "New access token in Authorization header, new refresh token in Set-Cookie", body = RotateRefreshTokenResponseData),
        (status = 401, description = "Missing or invalid refresh token cookie"),
    )
)]
#[instrument(skip(app_state, jar))]
pub async fn rotate_refresh_token_handler(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling rotate refresh token request");

    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized {
            message: "missing refresh token".to_string(),
        })?;

    let case = RotateRefreshTokenCase::new(
        app_state.user_repo.clone(),
        app_state.access_token_service.clone(),
        app_state.refresh_token_service.clone(),
        app_state.refresh_token_repo.clone(),
    );
    let result = case
        .execute(RotateRefreshTokenCommand { refresh_token })
        .await?;

    let cookie = Cookie::build(("refresh_token", result.refresh_token.value()))
        .path("/")
        .max_age(time::Duration::days(
            app_state.refresh_token_service.expires_in_days,
        ))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build();

    let mut headers = HeaderMap::new();
    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", result.access_token.value())).unwrap(),
    );
    headers.append(SET_COOKIE, cookie.to_string().parse().unwrap());

    let response = ApiResponse::<RotateRefreshTokenResponseData>::ok(
        Some(headers),
        "Rotate refresh token successfully",
        RotateRefreshTokenResponseData::from(&result),
    );
    tracing::info!("handling rotate refresh token request successfully");
    Ok(response)
}
