use crate::domain::auth::value_objects::tokens::access_token::{AccessToken, AccessTokenClaims};
use crate::domain::error::DomainResult;
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::user::user_id::UserId;

pub trait AccessTokenService: Send + Sync {
    fn generate(&self, user_id: &UserId, ver: &AccessTokenVersion) -> DomainResult<AccessToken>;
    fn decode(&self, access_token: &AccessToken) -> DomainResult<AccessTokenClaims>;
}
