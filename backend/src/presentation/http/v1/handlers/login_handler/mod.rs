pub mod request;
pub mod response;

use axum::response::IntoResponse;

pub async fn login_handler() -> impl IntoResponse {}
