use crate::domain::auth::service::access_token_service::{AccessClaims, AccessTokenService};
use crate::domain::auth::value_objects::tokens::access_token::AccessToken;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::infrastructure::errors::access_token_service_error::AccessTokenServiceError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};

pub struct AccessTokenServiceImplementation {
    secret: String,
}

impl AccessTokenServiceImplementation {
    pub fn new(secret: impl Into<String>) -> Self {
        AccessTokenServiceImplementation {
            secret: secret.into(),
        }
    }
}

impl AccessTokenService for AccessTokenServiceImplementation {
    fn encode_access_token(&self, user_id: UserId) -> Result<AccessToken, AccessTokenServiceError> {
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
        .map_err(|_| AccessTokenServiceError::EncodeAccessTokenFailed)?;
        Ok(AccessToken::new(access_token))
    }
    fn decode_access_token(
        &self,
        token: AccessToken,
    ) -> Result<AccessClaims, AccessTokenServiceError> {
        let access_token_data = decode::<AccessClaims>(
            token.value(),
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Default::default(),
        )
        .map_err(|_| AccessTokenServiceError::DecodeAccessTokenFailed)?;
        Ok(access_token_data.claims)
    }
}
