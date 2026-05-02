use snafu::Snafu;

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum InfrastructureError {
    
    #[snafu(display("Failed to load config: {source}"))]
    ConfigError {
        #[snafu(source(from(figment2::error::Error, Box::new)))]
        source: Box<figment2::error::Error>,
    },

    
    #[snafu(display("Failed to connect to PostgreSQL: {source}"))]
    PostgresError {
        #[snafu(source(from(sqlx::Error, Box::new)))]
        source: Box<sqlx::Error>,
    },
    #[snafu(display("Failed to run PostgreSQL migrations: {source}"))]
    PostgresMigrateError {
        #[snafu(source(from(sqlx::migrate::MigrateError, Box::new)))]
        source: Box<sqlx::migrate::MigrateError>,
    },

    
    #[snafu(display("Failed to connect to Redis: {source}"))]
    RedisError {
        #[snafu(source(from(redis::RedisError, Box::new)))]
        source: Box<redis::RedisError>,
    },
}
