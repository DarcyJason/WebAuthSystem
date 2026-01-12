use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors(frontend_address: String) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(frontend_address.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE])
        .allow_credentials(true)
}
