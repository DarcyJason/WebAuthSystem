use crate::domain::common::error::RepoSource;
use crate::infrastructure::layered::cache_layer::CacheLayer;
use crate::infrastructure::layered::cache_operation::CacheOperation;
use snafu::Snafu;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DomainError {
    // Identities Domain Error
    // == identities Domain Error ==
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
    #[snafu(
        visibility(pub),
        display("User password hash can't be same as last password hash: {new_password_hash}")
    )]
    UserPasswordHashMatched { new_password_hash: String },
    #[snafu(visibility(pub), display("User '{user_id}' not found"))]
    UserNotFound { user_id: String },
    #[snafu(visibility(pub), display("password {password} is invalid: {message}"))]
    InvalidPlainPassowrd { password: String, message: String },
    // == User Repository Error ==
    #[snafu(visibility(pub), display("UserRepository error at db: {message}"))]
    UserRepositoryDbError {
        #[snafu(source(from(sqlx::Error, |e| RepoSource::DB { source: Box::new(e) })))]
        source: RepoSource,
        message: String,
    },
    #[snafu(
        visibility(pub),
        display("UserRepository error at {layer} during {operation}: {message}")
    )]
    UserRepositoryRedisError {
        #[snafu(source(from(redis::RedisError, |e| RepoSource::Redis { source: Box::new(e) })))]
        source: RepoSource,
        layer: CacheLayer,
        operation: CacheOperation,
        message: String,
    },
    #[snafu(
        visibility(pub),
        display("UserRepository error at {layer} during {operation}: {message}")
    )]
    UserRepositoryJsonError {
        #[snafu(source(from(serde_json::Error, |e| RepoSource::Json { source: Box::new(e) })))]
        source: RepoSource,
        layer: CacheLayer,
        operation: CacheOperation,
        message: String,
    },
    // Auth Domain Error
    // == Password Service Error ==
    #[snafu(visibility(pub), display("hash password failed: {message}"))]
    HashPasswordFailed { message: String },
    #[snafu(visibility(pub), display("parse hashed password failed: {message}"))]
    ParsedHashedPasswordFailed { message: String },
    // == Email Service Error ==
    #[snafu(visibility(pub), display("system owner email invalid: {message}"))]
    SystemOwnerEmailInvalid { message: String },
    #[snafu(visibility(pub), display("send email failed: {message}"))]
    SendEmailFailed { message: String },
}
