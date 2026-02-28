use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};

use crate::domain::auth::services::token_service::{
    AccessClaims, AuthAccessTokenService, AuthAccessTokenServiceError,
};
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::user::value_objects::user_id::UserId;

pub struct AccessTokenService {
    secret: String,
}

impl AccessTokenService {
    pub fn new(secret: impl Into<String>) -> Self {
        AccessTokenService {
            secret: secret.into(),
        }
    }
}

impl AuthAccessTokenService for AccessTokenService {
    fn encode_access_token(
        &self,
        user_id: UserId,
    ) -> Result<AccessToken, AuthAccessTokenServiceError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(15)).timestamp() as usize;
        let access_claims = AccessClaims {
            sub: user_id,
            iat,
            exp,
        };
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AuthAccessTokenServiceError::EncodeAccessTokenFailed)?;
        Ok(AccessToken::new(access_token))
    }
    fn decode_access_token(
        &self,
        token: AccessToken,
    ) -> Result<AccessClaims, AuthAccessTokenServiceError> {
        let access_token_data = decode::<AccessClaims>(
            token.value(),
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Default::default(),
        )
        .map_err(|_| AuthAccessTokenServiceError::DecodeAccessTokenFailed)?;
        Ok(access_token_data.claims)
    }
}
