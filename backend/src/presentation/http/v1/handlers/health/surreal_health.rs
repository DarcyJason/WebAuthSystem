use axum::response::IntoResponse;
use crate::application::handlers::health::surreal_health_handler::SurrealHealthHandler;
use crate::infrastructure::persistence::surreal::health_repository::SurrealHealthRepository;
use crate::presentation::http::v1::response::ApiResponse;
use crate::presentation::http::v1::result::AppResult;

pub async fn surreal_health_handler(
) -> AppResult<impl IntoResponse> {
    let surreal_health_repo = SurrealHealthRepository::new();
    let handler = SurrealHealthHandler::new(surreal_health_repo);
    let data = handler.handle().await?;
    let response = ApiResponse::<()>::ok(200, "Surreal is Healthy", data);
    Ok(response)
}
