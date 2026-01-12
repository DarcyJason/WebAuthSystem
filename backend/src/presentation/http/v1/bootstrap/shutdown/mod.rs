use tracing::info;
use crate::presentation::http::v1::bootstrap::shutdown::clean::cleanup;

pub mod clean;

pub fn shutdown() {
    println!();
    info!("Starting graceful shutdown...");
    cleanup();
    info!("Shutdown complete");
}