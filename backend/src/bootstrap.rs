use std::sync::Arc;

use crate::app_config::AppConfig;
use crate::app_logo::show_app_logo;
use crate::app_state::AppState;
use crate::domain::auth::repositories::cache::email_verify_cache::EmailVerifyCache;
use crate::domain::auth::repositories::db::user_repo::UserRepository;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::auth::services::password_service::AuthPasswordService;
use crate::domain::auth::services::token_service::{
    AuthAccessTokenService, AuthRefreshTokenService,
};
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::redis::email_verify_cache::RedisEmailVerifyCache;
use crate::infrastructure::mail::MailService;
use crate::infrastructure::observability::logger::init_logger;
use crate::infrastructure::persistences::surrealdb::client::SurrealDBClient;
use crate::infrastructure::persistences::surrealdb::user_repo::SurrealDBUserRepository;
use crate::infrastructure::security::password::PasswordService;
use crate::infrastructure::security::tokens::access_token::AccessTokenService;
use crate::infrastructure::security::tokens::refresh_token::RefreshTokenService;
use crate::presentation::http::v1::middlewares::cors_middleware::cors_middleware;
use crate::presentation::http::v1::middlewares::trace_middleware::trace_middleware;
use crate::presentation::http::v1::openapi::ApiDoc;
use crate::presentation::http::v1::routers::build_routers;
use resend_rs::Resend;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn bootstrap() -> anyhow::Result<()> {
    show_app_logo();
    init_logger();
    let app_config = AppConfig::init()?;
    let backend_ip = app_config.server.backend_ip.clone();
    let backend_port = app_config.server.backend_port;
    let is_development = app_config.server.is_development_mode;
    let backend_address = format!("{}:{}", backend_ip, backend_port);
    let frontend_address = app_config.server.frontend_address.clone();
    let surrealdb_client = SurrealDBClient::new(&app_config.surrealdb).await?;
    let redis_client = RedisClient::new(&app_config.redis).await?;
    let mail_client = Resend::new(&app_config.resend.api_key);
    let user_repo: Arc<dyn UserRepository> =
        Arc::new(SurrealDBUserRepository::new(surrealdb_client));
    let auth_access_token_service: Arc<dyn AuthAccessTokenService> =
        Arc::new(AccessTokenService::new(app_config.jwt.secret));
    let auth_refresh_token_service: Arc<dyn AuthRefreshTokenService> =
        Arc::new(RefreshTokenService::new());
    let auth_password_service: Arc<dyn AuthPasswordService> = Arc::new(PasswordService::new());
    let auth_mail_service: Arc<dyn AuthMailService> = Arc::new(MailService::new(mail_client));
    let email_verify_cache: Arc<dyn EmailVerifyCache> =
        Arc::new(RedisEmailVerifyCache::new(redis_client));
    let app_state = AppState {
        user_repo,
        auth_access_token_service,
        auth_refresh_token_service,
        auth_password_service,
        auth_mail_service,
        email_verify_cache,
    };
    let listener = TcpListener::bind(backend_address).await?;
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
