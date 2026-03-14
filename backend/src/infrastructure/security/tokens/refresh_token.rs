use uuid::Uuid;

use crate::domain::auth::service::refresh_token_service::RefreshTokenService;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::infrastructure::errors::refresh_token_service_error::RefreshTokenServiceError;

pub struct RefreshTokenServiceImplementation;

impl RefreshTokenServiceImplementation {
    pub fn new() -> Self {
        RefreshTokenServiceImplementation {}
    }
}

impl Default for RefreshTokenServiceImplementation {
    fn default() -> Self {
        Self::new()
    }
}

impl RefreshTokenService for RefreshTokenServiceImplementation {
    fn generate_refresh_token(&self) -> Result<RefreshToken, RefreshTokenServiceError> {
        Ok(RefreshToken::new(Uuid::new_v4().to_string()))
    }
}
