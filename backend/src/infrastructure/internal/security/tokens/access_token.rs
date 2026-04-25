use crate::domain::auth::services::access_token_service::AccessTokenService;
use crate::domain::auth::value_objects::tokens::access_token::{AccessToken, AccessTokenClaims};
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::error::{
    DecodeAccessTokenFailedSnafu, DomainResult, EncodeAccessTokenFailedSnafu,
    InvalidAccessTokenSnafu,
};
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::user::user_id::UserId;
use chrono::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Debug, Clone)]
pub struct DefaultAccessTokenService {
    jwt_secret: String,
    expires_in_seconds: i64,
}

impl DefaultAccessTokenService {
    pub fn new(jwt_secret: impl Into<String>, expires_in_seconds: i64) -> Self {
        DefaultAccessTokenService {
            jwt_secret: jwt_secret.into(),
            expires_in_seconds,
        }
    }
}

impl AccessTokenService for DefaultAccessTokenService {
    fn generate(&self, user_id: &UserId, ver: &AccessTokenVersion) -> DomainResult<AccessToken> {
        let now = Timestamp::now().value().to_owned();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(self.expires_in_seconds)).timestamp() as usize;
        let claims = AccessTokenClaims::new(
            user_id.value().to_string(),
            ver.value().to_string(),
            iat,
            exp,
        );
        let access_token_value = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            EncodeAccessTokenFailedSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        Ok(AccessToken::new(access_token_value))
    }
    fn decode(&self, access_token: &AccessToken) -> DomainResult<AccessTokenClaims> {
        let decode = decode::<AccessTokenClaims>(
            access_token.value(),
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| {
            DecodeAccessTokenFailedSnafu {
                message: e.to_string(),
            }
            .build()
        });
        match decode {
            Ok(token) => Ok(token.claims),
            Err(e) => InvalidAccessTokenSnafu {
                message: e.to_string(),
            }
            .fail(),
        }
    }
}
