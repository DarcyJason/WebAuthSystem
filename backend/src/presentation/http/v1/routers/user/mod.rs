use axum::{Router, routing::get};

use crate::{
    app_state::AppState,
    presentation::http::v1::handlers::user::get_me_handler::handler::get_me_handler,
};

pub fn user_routers(app_state: AppState) -> Router {
    let user_routers = Router::new()
        .route("/me", get(get_me_handler))
        .with_state(app_state);
    Router::new().nest("/user", user_routers)
}
