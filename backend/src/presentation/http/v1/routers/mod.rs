use crate::presentation::http::v1::routers::auth::auth_routers;
use crate::presentation::http::v1::routers::health::health_routers;
use crate::presentation::http::v1::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod auth;
pub mod health;
pub mod user;

pub fn create_routers(app_state: Arc<AppState>) -> Router {
    let v1_routers = Router::new()
        .merge(health_routers(app_state.clone()))
        .merge(auth_routers(app_state.clone()));
    Router::new().nest("/api/v1", v1_routers)
}
