use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn cors_middleware(frontend_address: String) -> CorsLayer {
    let normalized_frontend = frontend_address.trim_end_matches('/').to_string();

    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _request_parts| {
                let origin_str = match origin.to_str() {
                    Ok(v) => v.trim_end_matches('/'),
                    Err(_) => return false,
                };

                origin_str == normalized_frontend
                    || origin_str == "http://localhost:5173"
                    || origin_str == "http://127.0.0.1:5173"
            },
        ))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE])
        .allow_credentials(true)
}
