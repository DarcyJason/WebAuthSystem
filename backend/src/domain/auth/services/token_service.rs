use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::{
    auth::value_objects::{access_token::AccessToken, refresh_token::RefreshToken},
    user::value_objects::user_id::UserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: UserId,
    // pub role: Vec<RoleCode>,
    // pub permission: Vec<PermissionCode>,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Error)]
pub enum AccessTokenServiceError {
    #[error("encode access token failed")]
    EncodeAccessTokenFailed,
    #[error("decode access token failed")]
    DecodeAccessTokenFailed,
}

pub trait AuthAccessTokenService: Send + Sync {
    fn encode_access_token(&self, user_id: UserId) -> Result<AccessToken, AccessTokenServiceError>;
    fn decode_access_token(
        &self,
        token: AccessToken,
    ) -> Result<AccessClaims, AccessTokenServiceError>;
}

#[derive(Debug, Error)]
pub enum RefreshTokenServiceError {
    #[error("generate refresh token failed")]
    GenerateRefreshTokenFailed,
}

pub trait AuthRefreshTokenService: Send + Sync {
    fn generate_refresh_token(&self) -> Result<RefreshToken, RefreshTokenServiceError>;
}
