use ntex::web;

use crate::handlers::auth::{login_handler, logout_handler, refresh_handler, register_handler};

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register_handler)))
            .service(web::resource("/login").route(web::post().to(login_handler)))
            .service(web::resource("/logout").route(web::post().to(logout_handler)))
            .service(web::resource("/refresh").route(web::post().to(refresh_handler))),
    );
}
