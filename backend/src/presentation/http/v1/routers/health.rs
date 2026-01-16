use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::presentation::http::v1::handlers::health::health_handler;
use crate::presentation::http::v1::handlers::health::surreal_health::surreal_health_handler;
use crate::presentation::http::v1::state::AppState;

pub fn health_routers(app_state: Arc<AppState>) -> Router {
    let health_routers = Router::new()
        .route("/", get(health_handler))
        .route("/surreal", get(surreal_health_handler))
        .with_state(app_state);
    Router::new().nest("/health", health_routers)
}