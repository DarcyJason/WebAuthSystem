use ntex::web;

use crate::handlers::user::me_handler;

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/me", web::get().to(me_handler));
}
