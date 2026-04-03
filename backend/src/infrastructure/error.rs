use snafu::{Backtrace, Location, Snafu};

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum InfrastructureError {
    #[snafu(visibility(pub), display("Failed to load config: {}", source))]
    Config {
        #[snafu(source(from(figment2::error::Error, Box::new)))]
        source: Box<figment2::error::Error>,
        #[snafu(implicit)]
        backtrace: Backtrace,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(visibility(pub), display("Failed to connect to postgres: {}", source))]
    Postgres {
        #[snafu(source(from(sqlx::Error, Box::new)))]
        source: Box<sqlx::Error>,
        #[snafu(implicit)]
        backtrace: Backtrace,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(visibility(pub), display("Failed to connect to redis: {}", source))]
    Redis {
        #[snafu(source(from(redis::RedisError, Box::new)))]
        source: Box<redis::RedisError>,
        #[snafu(implicit)]
        backtrace: Backtrace,
        #[snafu(implicit)]
        location: Location,
    },
}
