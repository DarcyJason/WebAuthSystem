use ntex::web;

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/protected"));
}
