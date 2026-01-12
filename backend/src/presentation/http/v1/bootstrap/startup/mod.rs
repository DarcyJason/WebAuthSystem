use tracing::info;
use crate::presentation::http::v1::bootstrap::startup::logger::logger;
use crate::presentation::http::v1::bootstrap::startup::logo::show_brand_logo;

pub mod logger;
pub mod logo;

pub fn startup() {
    show_brand_logo();
    logger();
    info!("Application initialized successfully");
}