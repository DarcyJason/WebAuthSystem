use utoipa::OpenApi;

use crate::presentation::http::v1::handlers::auth::login::login_handler::__path_login_handler;
use crate::presentation::http::v1::handlers::auth::register::register_handler::__path_register_handler;
use crate::presentation::http::v1::handlers::health::backend_health::backend_health_handler::__path_backend_health_handler;
use crate::presentation::http::v1::handlers::health::redis_health::redis_health_handler::__path_redis_health_handler;
use crate::presentation::http::v1::handlers::health::surreal_health::surreal_health_handler::__path_surreal_health_handler;
use crate::presentation::http::v1::handlers::user::get_me::get_me_handler::__path_get_me_handler;

#[derive(OpenApi)]
#[openapi(
    info(description = "Backend API"),
    paths(
        backend_health_handler,
        redis_health_handler,
        surreal_health_handler,
        register_handler,
        login_handler,
        get_me_handler,
),
    tags(
        (name = "Health", description = "Health API"),
        (name = "Auth", description = "Auth API"),
        (name = "User", description = "User API"),
    ))]
pub struct ApiDoc;
