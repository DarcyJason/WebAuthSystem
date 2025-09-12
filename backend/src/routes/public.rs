use ntex::web;

use crate::handlers::auth::register_handler;

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register_handler))),
    );
}
