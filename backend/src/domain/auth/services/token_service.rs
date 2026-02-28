use serde::{Deserialize, Serialize};

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

pub enum AuthAccessTokenServiceError {
    EncodeAccessTokenFailed,
    DecodeAccessTokenFailed,
}

pub trait AuthAccessTokenService: Send + Sync {
    fn encode_access_token(
        &self,
        user_id: UserId,
    ) -> Result<AccessToken, AuthAccessTokenServiceError>;
    fn decode_access_token(
        &self,
        token: AccessToken,
    ) -> Result<AccessClaims, AuthAccessTokenServiceError>;
}

pub enum AuthRefreshTokenServiceError {
    GenerateRefreshTokenFailed,
}

pub trait AuthRefreshTokenService: Send + Sync {
    fn generate_refresh_token(&self) -> Result<RefreshToken, AuthRefreshTokenServiceError>;
}
