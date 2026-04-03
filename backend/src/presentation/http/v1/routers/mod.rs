use crate::application::app_state::AppState;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn build_routers(app_state: Arc<AppState>) -> Router {
    Router::new().layer(Extension(app_state))
}
