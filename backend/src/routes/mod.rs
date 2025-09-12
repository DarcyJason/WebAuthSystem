use ntex::web;

use crate::routes::{protected::protected_routes, public::public_routes};

pub mod protected;
pub mod public;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(public_routes)
            .configure(protected_routes),
    );
}
