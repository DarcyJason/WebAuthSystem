use crate::infrastructure::error::InfrastructureError;
use snafu::Snafu;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ApplicationError {
    #[snafu(display("Database error: {}", source))]
    Postgres { source: InfrastructureError },
    #[snafu(display("Redis error: {}", source))]
    Redis { source: InfrastructureError },
}
