use crate::application::app_state::AppState;
use crate::infrastructure::{config::Config, logger::init_logger, logo::print_logo};
use crate::presentation::http::v1::middlewares::cors::cors_middleware;
use crate::presentation::http::v1::middlewares::trace::trace_middleware;
use crate::presentation::http::v1::openapi::ApiDoc;
use crate::presentation::http::v1::routers::build_routers;
use snafu::{ResultExt, Whatever};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub async fn run() -> Result<(), Whatever> {
    let _gaurd = init_logger();
    let config = Config::init().with_whatever_context(|_| "Failed to read config files")?;
    print_logo();
    let backend_ip = config.server.backend_ip.clone();
    let backend_port = config.server.backend_port;
    let is_development = config.server.is_development_mode;
    let backend_address = format!("{}:{}", backend_ip, backend_port);
    let frontend_address = config.server.frontend_address.clone();
    let listener = TcpListener::bind(backend_address)
        .await
        .with_whatever_context(|_| "Failed to bind TCP listener")?;
    let app_state = Arc::new(
        AppState::new(config.clone())
            .await
            .with_whatever_context(|_| "Failed to init app_state")?,
    );
    let mut app = build_routers(app_state)
        .layer(cors_middleware(frontend_address))
        .layer(trace_middleware());
    tracing::info!(
        "axum server is listening on http://localhost:{}",
        backend_port
    );
    if is_development {
        tracing::info!(
            "Swagger UI doc is on http://localhost:{}/swagger",
            backend_port
        );
        app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_whatever_context(|_| "Failed to shutdown the axum server")?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!();
    tracing::info!("signal received, starting graceful shutdown");
}
