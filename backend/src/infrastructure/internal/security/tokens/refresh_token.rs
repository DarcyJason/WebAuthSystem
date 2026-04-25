use crate::domain::auth::services::refresh_token_service::RefreshTokenService;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DefaultRefreshTokenService {
    pub expires_in_days: i64,
}

impl DefaultRefreshTokenService {
    pub fn new(expires_in_days: i64) -> Self {
        DefaultRefreshTokenService { expires_in_days }
    }
}

impl RefreshTokenService for DefaultRefreshTokenService {
    fn generate(&self) -> RefreshToken {
        RefreshToken::new(Uuid::new_v4().to_string())
    }
}
