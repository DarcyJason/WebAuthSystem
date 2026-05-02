use crate::infrastructure::internal::layered::cache_layer::CacheLayer;
use crate::infrastructure::internal::layered::cache_operation::CacheOperation;
use snafu::Snafu;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DomainError {
    
    #[snafu(visibility(pub), display("Invalid user_id '{user_id}': {source}"))]
    InvalidUserId {
        user_id: String,
        #[snafu(source(from(uuid::Error, Box::new)))]
        source: Box<uuid::Error>,
    },
    #[snafu(visibility(pub), display("Invalid user_name '{user_name}': {message}"))]
    InvalidUserName { user_name: String, message: String },
    #[snafu(
        visibility(pub),
        display("Invalid user_email '{user_email}': {message}")
    )]
    InvalidUserEmail { user_email: String, message: String },
    #[snafu(visibility(pub), display("Invalid user_name or user_email: {message}"))]
    InvalidUserNameOrUserEmail { message: String },
    #[snafu(
        visibility(pub),
        display("Invalid plain password '{password}': {message}")
    )]
    InvalidPlainPassword { password: String, message: String },
    #[snafu(visibility(pub), display("User '{user_id}' not found"))]
    UserNotFound { user_id: String },
    #[snafu(
        visibility(pub),
        display("New password hash must differ from the current one: {new_password_hash}")
    )]
    UserPasswordHashMatched { new_password_hash: String },

    
    #[snafu(visibility(pub), display("UserRepository db error: {message}"))]
    UserRepositoryDb {
        #[snafu(source(from(sqlx::Error, Box::new)))]
        source: Box<sqlx::Error>,
        message: String,
    },
    #[snafu(
        visibility(pub),
        display("UserRepository cache error at {layer} during {operation}: {message}")
    )]
    UserRepositoryRedis {
        #[snafu(source(from(redis::RedisError, Box::new)))]
        source: Box<redis::RedisError>,
        layer: CacheLayer,
        operation: CacheOperation,
        message: String,
    },
    #[snafu(
        visibility(pub),
        display(
            "UserRepository cache serialization error at {layer} during {operation}: {message}"
        )
    )]
    UserRepositoryJson {
        #[snafu(source(from(serde_json::Error, Box::new)))]
        source: Box<serde_json::Error>,
        layer: CacheLayer,
        operation: CacheOperation,
        message: String,
    },

    
    #[snafu(visibility(pub), display("RefreshTokenRepository db error: {message}"))]
    RefreshTokenRepositoryDb {
        #[snafu(source(from(sqlx::Error, Box::new)))]
        source: Box<sqlx::Error>,
        message: String,
    },

    
    #[snafu(
        visibility(pub),
        display("VerificationTokenRepository db error: {message}")
    )]
    VerificationTokenRepositoryDb {
        #[snafu(source(from(sqlx::Error, Box::new)))]
        source: Box<sqlx::Error>,
        message: String,
    },

    
    #[snafu(visibility(pub), display("Verification token not found"))]
    VerificationTokenNotFound,
    #[snafu(visibility(pub), display("Verification token has expired"))]
    VerificationTokenExpired,
    #[snafu(visibility(pub), display("Verification token has already been used"))]
    VerificationTokenAlreadyUsed,

    
    #[snafu(visibility(pub), display("Failed to hash password: {message}"))]
    HashPasswordFailed { message: String },
    #[snafu(visibility(pub), display("Failed to parse hashed password: {message}"))]
    ParsedHashedPasswordFailed { message: String },

    
    #[snafu(visibility(pub), display("System owner email is invalid: {message}"))]
    SystemOwnerEmailInvalid { message: String },
    #[snafu(visibility(pub), display("Failed to send email: {message}"))]
    SendEmailFailed { message: String },

    
    #[snafu(visibility(pub), display("Failed to encode access token: {message}"))]
    EncodeAccessTokenFailed { message: String },
    #[snafu(visibility(pub), display("Failed to decode access token: {message}"))]
    DecodeAccessTokenFailed { message: String },
    #[snafu(visibility(pub), display("Invalid access token: {message}"))]
    InvalidAccessToken { message: String },
}
