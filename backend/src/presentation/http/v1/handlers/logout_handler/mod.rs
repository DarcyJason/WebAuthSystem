pub mod request;
pub mod response;

use axum::response::IntoResponse;

pub async fn logout_handler() -> impl IntoResponse {}
