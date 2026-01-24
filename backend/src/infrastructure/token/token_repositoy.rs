use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::infrastructure::token::claims::AccessClaims;
use crate::infrastructure::token::errors::TokenError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use surrealdb::RecordId;
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
        let claims = AccessClaims {
            sub: user_id.to_string(),
            exp: 15,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| TokenError::EncodeJWTError)?;
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
        .map_err(|_| TokenError::DecodeJWTError)?;
        Ok(token_data.claims)
    }
}
