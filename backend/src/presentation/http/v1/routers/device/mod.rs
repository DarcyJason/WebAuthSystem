use axum::Router;

use crate::presentation::http::v1::states::app_state::AppState;

pub fn device_routers(app_state: AppState) -> Router {
    let device_routers = Router::new().with_state(app_state);
    Router::new().nest("/device", device_routers)
}
