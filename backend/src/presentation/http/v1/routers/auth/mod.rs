use axum::{routing::post, Router};

use crate::presentation::http::v1::handlers::auth::validate_verification_handler::handler::validate_verification_handler;
use crate::presentation::http::v1::handlers::auth::{
    login_handler::handler::login_handler, register_handler::handler::register_handler,
    send_verification_email_handler::handler::send_verification_email_handler,
};

pub fn auth_routers() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/send", post(send_verification_email_handler))
        .route("/validate", post(validate_verification_handler))
}
