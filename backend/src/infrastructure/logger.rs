use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("./logs", "backend.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_writer(non_blocking);
    let console_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "backend=info".into()))
        .with(file_layer)
        .with(console_layer)
        .init();
    guard
}
