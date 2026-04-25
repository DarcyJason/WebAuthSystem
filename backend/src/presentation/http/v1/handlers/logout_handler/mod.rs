pub mod response;

use crate::application::app_state::AppState;
use crate::application::use_cases::logout_case::LogoutCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::logout_handler::response::LogoutResponseData;
use crate::presentation::http::v1::middlewares::auth::AuthMiddleware;
use crate::presentation::http::v1::response::ApiResponse;
use axum::extract::{Extension, State};
use axum::http::HeaderMap;
use axum::http::header::SET_COOKIE;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Auth",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "Logged out — refresh token cookie cleared", body = LogoutResponseData),
        (status = 401, description = "Unauthorized"),
    )
)]
#[instrument(skip(app_state), fields(user_id=%auth.user.id()))]
pub async fn logout_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthMiddleware>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling logout request");
    let case = LogoutCase::new(app_state.user_repo.clone());
    let result = case.execute(auth.user.id()).await?;
    let cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build();
    let mut response_header = HeaderMap::new();
    response_header.append(SET_COOKIE, cookie.to_string().parse().unwrap());
    let response_data = LogoutResponseData::from(result);
    let response = ApiResponse::<LogoutResponseData>::ok(
        Some(response_header),
        "Logout successfully",
        response_data,
    );
    tracing::info!("handling logout request successfully");
    Ok(response)
}
