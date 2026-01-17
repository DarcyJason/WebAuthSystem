use crate::application::use_cases::health::redis_health_case::RedisHealthCase;
use crate::infrastructure::cache::redis::health_repository::RedisHealthRepository;
use crate::presentation::http::v1::{errors::ApiResult, response::ApiResponse, state::AppState};
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn redis_health_handler(
    State(app_state): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let redis_health_repo = RedisHealthRepository::new(app_state.redis.clone());
    let case = RedisHealthCase::new(redis_health_repo);
    let (message, data) = case.execute().await?;
    let response = ApiResponse::<()>::ok(200, message, data);
    Ok(response)
}
