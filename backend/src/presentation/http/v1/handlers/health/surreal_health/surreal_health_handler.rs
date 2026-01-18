use crate::application::use_cases::health::surreal_health_case::SurrealHealthCase;
use crate::infrastructure::persistence::surreal::health_repository::SurrealHealthRepository;
use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::response::ApiResponse;
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument]
pub async fn surreal_health_handler() -> ApiResult<impl IntoResponse> {
    let surreal_health_repo = SurrealHealthRepository::new();
    let case = SurrealHealthCase::new(surreal_health_repo);
    let (message, data) = case.execute().await?;
    let response = ApiResponse::<()>::ok(200, message, data);
    Ok(response)
}
