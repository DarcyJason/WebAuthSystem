use crate::presentation::http::v1::routers::{
    admin::admin_routers, auth::auth_routers, device::device_routers, user::user_routers,
};
use crate::presentation::http::v1::states::AppState;
use axum::{Extension, Router};
use std::sync::Arc;

pub mod admin;
pub mod auth;
pub mod device;
pub mod user;

pub fn build_routers(app_state: Arc<AppState>) -> Router {
    let all_routers = Router::new()
        .nest("/admin", admin_routers())
        .nest("/auth", auth_routers())
        .nest("/device", device_routers())
        .nest("/user", user_routers())
        .layer(Extension(app_state));
    Router::new().nest("/api/v1", all_routers)
}
