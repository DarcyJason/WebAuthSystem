use crate::application::app_state::AppState;
use crate::presentation::http::v1::handlers::forgot_password_handler::forgot_password_handler;
use crate::presentation::http::v1::handlers::login_handler::login_handler;
use crate::presentation::http::v1::handlers::logout_handler::logout_handler;
use crate::presentation::http::v1::handlers::register_handler::register_handler;
use crate::presentation::http::v1::handlers::resend_verification_handler::resend_verification_handler;
use crate::presentation::http::v1::handlers::reset_password_handler::reset_password_handler;
use crate::presentation::http::v1::handlers::rotate_refresh_token_handler::rotate_refresh_token_handler;
use crate::presentation::http::v1::handlers::verify_handler::verify_handler;
use crate::presentation::http::v1::middlewares::auth::auth;
use axum::Router;
use axum::middleware;
use axum::routing::post;
use std::sync::Arc;

pub fn auth_routers(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/resend-verification", post(resend_verification_handler))
        .route("/login", post(login_handler))
        .route("/forgot-password", post(forgot_password_handler))
        .route("/verify", post(verify_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/rotate-refresh-token", post(rotate_refresh_token_handler))
        .route(
            "/logout",
            post(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
}
