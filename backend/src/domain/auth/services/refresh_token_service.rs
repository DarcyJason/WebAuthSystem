use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::infrastructure::errors::refresh_token_service_error::RefreshTokenServiceError;

pub trait AuthRefreshTokenService: Send + Sync {
    fn generate_refresh_token(&self) -> Result<RefreshToken, RefreshTokenServiceError>;
}
