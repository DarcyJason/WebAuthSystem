use crate::presentation::http::v1::handlers::user::get_me::get_me_handler::get_me_handler;
use crate::presentation::http::v1::state::AppState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn user_routers(app_state: Arc<AppState>) -> Router {
    let user_routers = Router::new()
        .route("/me", get(get_me_handler))
        .with_state(app_state);
    Router::new().nest("/user", user_routers)
}
