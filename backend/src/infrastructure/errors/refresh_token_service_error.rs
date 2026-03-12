use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefreshTokenServiceError {
    #[error("Generate refresh token failed")]
    GenerateRefreshTokenFailed,
}
