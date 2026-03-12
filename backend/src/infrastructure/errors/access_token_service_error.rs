use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccessTokenServiceError {
    #[error("Encode access token failed")]
    EncodeAccessTokenFailed,
    #[error("Decode access token failed")]
    DecodeAccessTokenFailed,
}
