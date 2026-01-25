use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::infrastructure::token::claims::AccessClaims;
use crate::infrastructure::token::errors::TokenError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use surrealdb::RecordId;
use tracing::error;
use uuid::Uuid;

pub struct TokenRepository {
    pub secret: String,
}

impl TokenRepository {
    pub fn new(secret: &str) -> Self {
        TokenRepository {
            secret: secret.to_string(),
        }
    }
    pub fn generate_access_token(&self, user_id: RecordId) -> Result<AccessToken, TokenError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(15)).timestamp() as usize;
        let claims = AccessClaims {
            sub: user_id.to_string(),
            iat,
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| {
            error!("encode access_token error: {:?}", e);
            TokenError::EncodeJWTError
        })?;
        Ok(AccessToken::new(token))
    }
    pub fn generate_refresh_token(&self) -> Result<RefreshToken, TokenError> {
        Ok(RefreshToken::new(Uuid::new_v4().to_string()))
    }
    pub fn decode_access_token(&self, token: &str) -> Result<AccessClaims, TokenError> {
        let token_data = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Default::default(),
        )
        .map_err(|e| {
            error!("decode access_token error: {:?}", e);
            TokenError::DecodeJWTError
        })?;
        Ok(token_data.claims)
    }
}
