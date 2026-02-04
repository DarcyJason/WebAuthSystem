use uuid::Uuid;

use crate::domain::auth::{
    services::token_service::{AuthRefreshTokenService, AuthRefreshTokenServiceError},
    value_objects::refresh_token::RefreshToken,
};

pub struct RefreshTokenService {}

impl RefreshTokenService {
    pub fn new() -> Self {
        RefreshTokenService {}
    }
}

impl AuthRefreshTokenService for RefreshTokenService {
    fn generate_refresh_token(&self) -> Result<RefreshToken, AuthRefreshTokenServiceError> {
        Ok(RefreshToken::new(Uuid::new_v4().to_string()))
    }
}
