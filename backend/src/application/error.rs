use crate::domain::error::DomainError;
use crate::infrastructure::error::InfrastructureError;
use snafu::Snafu;

pub type ApplicationResult<T> = Result<T, ApplicationError>;


#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ApplicationError {
    
    #[snafu(display("Database error: {source}"))]
    PostgresFailed { source: InfrastructureError },
    #[snafu(display("Redis error: {source}"))]
    RedisFailed { source: InfrastructureError },

    
    #[snafu(display("Domain error: {source}"))]
    DomainFailed {
        #[snafu(source(from(DomainError, Box::new)))]
        source: Box<DomainError>,
    },
    #[snafu(display("{source}"))]
    PasswordServiceFailed {
        #[snafu(source(from(DomainError, Box::new)))]
        source: Box<DomainError>,
    },

    
    #[snafu(display("Validation failed: {message}"))]
    Validation { message: String },

    
    #[snafu(display("User already exists"))]
    UserAlreadyExists,
    #[snafu(display("User not found"))]
    UserNotFound,

    
    #[snafu(display("Invalid credentials"))]
    InvalidCredentials,
    #[snafu(display("Account is banned"))]
    AccountIsBanned,
    #[snafu(display("Email is not verified"))]
    EmailNotVerified,
    #[snafu(display("Passwords do not match"))]
    PasswordsNotMatched,
    #[snafu(display("Invalid or expired refresh token"))]
    InvalidRefreshToken,

    
    #[snafu(display("Verification token not found"))]
    VerificationTokenNotFound,
    #[snafu(display("Verification token has expired"))]
    VerificationTokenExpired,
    #[snafu(display("Verification token has already been used"))]
    VerificationTokenAlreadyUsed,
}
