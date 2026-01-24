use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid Credentials")]
    InvalidCredentials,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("user not found")]
    UserNotFound,
    #[error("generate access_token error")]
    GenerateAccessTokenFailed,
    #[error("generate refresh_token error")]
    GenerateRefreshTokenFailed,
    #[error("decode access_token error")]
    DecodeAccessTokenFailed,
}
