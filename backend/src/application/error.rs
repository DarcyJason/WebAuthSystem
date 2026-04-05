use crate::infrastructure::error::InfrastructureError;
use snafu::Snafu;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ApplicationError {
    #[snafu(visibility(pub), display("Database error: {}", source))]
    Postgres { source: InfrastructureError },
    #[snafu(visibility(pub), display("Redis error: {}", source))]
    Redis { source: InfrastructureError },
    #[snafu(visibility(pub), display("Field is invalid: {}", message))]
    FieldInvalid { message: String },
}
