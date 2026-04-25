pub mod response;

use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::get_me_handler::response::GetMeResponseData;
use crate::presentation::http::v1::middlewares::auth::AuthMiddleware;
use crate::presentation::http::v1::response::ApiResponse;
use axum::extract::Extension;
use axum::response::IntoResponse;
use tracing::instrument;

#[utoipa::path(
    get,
    path = "/api/v1/me",
    tag = "User",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current authenticated user info", body = GetMeResponseData),
        (status = 401, description = "Unauthorized"),
    )
)]
#[instrument(skip(auth), fields(user_id=%auth.user.id()))]
pub async fn get_me_handler(
    Extension(auth): Extension<AuthMiddleware>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling get me request");
    let response_data = GetMeResponseData::from(&auth);
    let response = ApiResponse::<GetMeResponseData>::ok(None, "Get me successfully", response_data);
    tracing::info!("handling get me request successfully");
    Ok(response)
}
