use axum::{Router, routing::post};

use crate::presentation::http::v1::{
    handlers::auth::{
        login_handler::handler::login_handler, register_handler::handler::register_handler,
        send_verification_email_handler::handler::send_verification_email_handler,
    },
    states::app_state::AppState,
};

pub fn auth_routers(app_state: AppState) -> Router {
    let auth_routers = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/send", post(send_verification_email_handler))
        .with_state(app_state);
    Router::new().nest("/auth", auth_routers)
}
