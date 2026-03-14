use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::infrastructure::errors::refresh_token_service_error::RefreshTokenServiceError;

pub trait RefreshTokenService: Send + Sync {
    fn generate_refresh_token(&self) -> Result<RefreshToken, RefreshTokenServiceError>;
}
