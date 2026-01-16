use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::presentation::http::v1::handlers::auth::register::register;
use crate::presentation::http::v1::state::AppState;

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    let auth_routers = Router::new()
        .route("/register", post(register))
        .with_state(app_state);
    Router::new().nest("/auth", auth_routers)
}