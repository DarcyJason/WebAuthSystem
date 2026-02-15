pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::infrastructure::config::Config;
use crate::infrastructure::observability::logger::init_logger;
use crate::presentation::http::v1::assets::app_logo::show_app_logo;
use crate::presentation::http::v1::middlewares::cors_middleware::cors_middleware;
use crate::presentation::http::v1::middlewares::trace_middleware::trace_middleware;
use crate::presentation::http::v1::openapi::ApiDoc;
use crate::presentation::http::v1::routers::build_routers;
use crate::presentation::http::v1::states::app_state::AppState;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn bootstrap() -> anyhow::Result<()> {
    show_app_logo();
    init_logger();
    let config = Config::init()?;
    let backend_ip = config.server.backend_ip.clone();
    let backend_port = config.server.backend_port;
    let is_development = config.server.is_development_mode;
    let backend_address = format!("{}:{}", backend_ip, backend_port);
    let frontend_address = config.server.frontend_address.clone();
    let listener = TcpListener::bind(backend_address).await?;
    let app_state = AppState::init(config.clone()).await?;
    let mut app = build_routers(app_state)
        .layer(cors_middleware(frontend_address))
        .layer(trace_middleware());
    info!(
        "axum server is listening on http://localhost:{}",
        backend_port
    );
    if is_development {
        info!(
            "Swagger UI doc is on http://localhost:{}/swagger",
            backend_port
        );
        app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
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
    info!("signal received, starting graceful shutdown");
}
