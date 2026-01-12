use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::presentation::http::v1::bootstrap::shutdown::shutdown;
use crate::presentation::http::v1::bootstrap::startup::startup;
use crate::presentation::http::v1::config::Config;
use crate::presentation::http::v1::middlewares::cors::cors;
use crate::presentation::http::v1::routers::create_routers;
use crate::presentation::http::v1::state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub async fn run() -> anyhow::Result<()> {
    startup();
    let config = Config::new()?;
    let frontend_address = config.server.frontend_address.clone();
    let backend_address = config.server.backend_address.clone();
    let surreal = SurrealClient::new(&config.surreal).await?;
    let app_state = AppState::new(config.clone(), surreal);
    let routers = create_routers(Arc::new(app_state))
        .layer(TraceLayer::new_for_http())
        .layer(cors(frontend_address));
    let listener = TcpListener::bind(backend_address.clone()).await?;
    info!("Axum server is listening on {}", backend_address);
    axum::serve(listener, routers)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c()
                .await
                .map_err(|e| error!("Failed to install CTRL+C signal handler: {}", e));
        })
        .await?;
    shutdown();
    Ok(())
}
