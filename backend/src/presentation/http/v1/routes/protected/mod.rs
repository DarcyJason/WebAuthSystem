use crate::application::app_state::AppState;
use crate::presentation::http::v1::handlers::change_password_handler::change_password_handler;
use crate::presentation::http::v1::handlers::get_me_handler::get_me_handler;
use crate::presentation::http::v1::middlewares::auth::auth;
use axum::Router;
use axum::middleware;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn protected_routers(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(get_me_handler))
        .route("/change-password", post(change_password_handler))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
}
