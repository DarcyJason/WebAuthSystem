use ntex::web;

use crate::{
    middlewares::auth::Auth, // 导入 Auth 中间件
    routes::{protected::protected_routes, public::public_routes},
};

pub mod protected;
pub mod public;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(public_routes)
            .service(web::scope("/user").wrap(Auth).configure(protected_routes)),
    );
}
