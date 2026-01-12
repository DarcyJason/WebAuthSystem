use crate::presentation::http::v1::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod health;
pub mod auth;
pub mod user;

pub fn create_routers(app_state: Arc<AppState>) -> Router {
    let v1_routers = Router::new().with_state(app_state);
    Router::new().nest("/api/v1", v1_routers)
}
