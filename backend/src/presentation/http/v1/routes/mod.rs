pub mod auth;
pub mod protected;

use crate::application::app_state::AppState;
use crate::presentation::http::v1::routes::auth::auth_routers;
use crate::presentation::http::v1::routes::protected::protected_routers;
use axum::Router;
use std::sync::Arc;

pub fn build_routers(app_state: Arc<AppState>) -> Router {
    let all_routers = Router::new()
        .nest("/auth", auth_routers(app_state.clone()))
        .nest("/protected", protected_routers(app_state.clone()))
        .with_state(app_state);
    Router::new().nest("/api/v1", all_routers)
}
