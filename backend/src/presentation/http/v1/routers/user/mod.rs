use axum::{Router, routing::get};

use crate::presentation::http::v1::handlers::user::get_me_handler::handler::get_me_handler;

pub fn user_routers() -> Router {
    Router::new().route("/me", get(get_me_handler))
}
