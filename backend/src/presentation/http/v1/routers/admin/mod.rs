use axum::Router;

use crate::presentation::http::v1::states::app_state::AppState;

pub fn admin_routers(app_state: AppState) -> Router {
    let admin_routers = Router::new().with_state(app_state);
    Router::new().nest("/admin", admin_routers)
}
