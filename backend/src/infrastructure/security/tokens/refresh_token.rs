use uuid::Uuid;

use crate::domain::auth::service::refresh_token_service::RefreshTokenService;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::infrastructure::errors::refresh_token_service_error::RefreshTokenServiceError;

pub struct DefaultRefreshTokenService;

impl DefaultRefreshTokenService {
    pub fn new() -> Self {
        DefaultRefreshTokenService {}
    }
}

impl Default for DefaultRefreshTokenService {
    fn default() -> Self {
        Self::new()
    }
}

impl RefreshTokenService for DefaultRefreshTokenService {
    fn generate_refresh_token(&self) -> Result<RefreshToken, RefreshTokenServiceError> {
        Ok(RefreshToken::new(Uuid::new_v4().to_string()))
    }
}
