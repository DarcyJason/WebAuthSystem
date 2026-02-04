use crate::{app_state::AppState, presentation::http::v1::routers::auth::auth_routers};
use axum::Router;

pub mod admin;
pub mod auth;
pub mod device;
pub mod user;

pub fn build_routers(app_state: AppState) -> Router {
    let all_routers = Router::new().merge(auth_routers(app_state.clone()));
    Router::new().nest("/api/v1", all_routers)
}
