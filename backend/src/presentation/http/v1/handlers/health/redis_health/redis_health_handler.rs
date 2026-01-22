use crate::application::use_cases::health::redis_health_case::RedisHealthCase;
use crate::infrastructure::cache::redis::health_cache::RedisHealthCache;
use crate::presentation::http::v1::{errors::ApiResult, response::ApiResponse, state::AppState};
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::{info, instrument};

#[instrument(skip(app_state))]
#[utoipa::path(get, path = "/api/v1/health/redis", responses(
    (status = 200, description = "Redis is healthy")
), tag = "Health")]
pub async fn redis_health_handler(
    State(app_state): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    info!("Start handling redis health successfully");
    let redis_health_cache = RedisHealthCache::new(app_state.redis.clone());
    let case = RedisHealthCase::new(redis_health_cache);
    case.execute().await?;
    let response = ApiResponse::<()>::ok(200, "Redis is healthy", ());
    info!("Finish handling redis health successfully");
    Ok(response)
}
