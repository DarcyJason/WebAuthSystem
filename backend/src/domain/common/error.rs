use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum RepoSource {
    #[snafu(display("Database error: {source}"))]
    DB {
        #[snafu(source)]
        source: Box<sqlx::Error>,
    },
    #[snafu(display("Redis error: {source}"))]
    Redis {
        #[snafu(source)]
        source: Box<redis::RedisError>,
    },
    #[snafu(display("JSON error: {source}"))]
    Json {
        #[snafu(source)]
        source: Box<serde_json::Error>,
    },
    #[snafu(display("Moka error: {source}"))]
    Moka {
        #[snafu(source)]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}
