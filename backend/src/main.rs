use backend::{observability::log::init_log, run};
use dotenvy::dotenv;
use tracing::error;

#[ntex::main]
async fn main() {
    dotenv().ok();

    let _guard = init_log();

    if let Err(e) = run().await {
        error!("Backend failed to start: {}", e);
        std::process::exit(1);
    }
}
