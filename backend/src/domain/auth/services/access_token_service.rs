use crate::domain::auth::value_objects::tokens::access_token::AccessToken;
use crate::domain::user::entities::user::user_id::UserId;
use crate::infrastructure::errors::access_token_service_error::AccessTokenServiceError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: UserId,
    // pub role: Vec<RoleCode>,
    // pub permission: Vec<PermissionCode>,
    pub iat: usize,
    pub exp: usize,
}

pub trait AccessTokenService: Send + Sync {
    fn encode_access_token(&self, user_id: UserId) -> Result<AccessToken, AccessTokenServiceError>;
    fn decode_access_token(
        &self,
        token: AccessToken,
    ) -> Result<AccessClaims, AccessTokenServiceError>;
}
