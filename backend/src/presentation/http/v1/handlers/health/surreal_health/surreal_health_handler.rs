use crate::application::use_cases::health::surreal_health_case::SurrealHealthCase;
use crate::infrastructure::persistence::surreal::health_repository::SurrealHealthRepository;
use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::response::ApiResponse;
use axum::response::IntoResponse;
use tracing::{info, instrument};

#[instrument]
#[utoipa::path(get, path = "/api/v1/health/surreal", responses(
    (status = 200, description = "SurrealDB is healthy")
), tag = "Health")]
pub async fn surreal_health_handler() -> ApiResult<impl IntoResponse> {
    info!("Start handling surreal health");
    let surreal_health_repo = SurrealHealthRepository::new();
    let case = SurrealHealthCase::new(surreal_health_repo);
    case.execute().await?;
    let response = ApiResponse::<()>::ok(200, "SurrealDB is healthy", ());
    info!("Finish handling surreal health");
    Ok(response)
}
