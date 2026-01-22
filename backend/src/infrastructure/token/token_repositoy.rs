use crate::domain::auth::errors::AuthError;
use crate::domain::auth::repositories::AuthTokenRepository;
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::errors::RepoResult;
use crate::infrastructure::token::claims::AccessClaims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
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
}

impl AuthTokenRepository for TokenRepository {
    fn generate_access_token(&self, user_id: RecordId) -> RepoResult<AccessToken> {
        let claims = AccessClaims {
            sub: user_id.to_string(),
            exp: 15,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        )
        .map_err(|_| AuthError::EncodeJWTError)?;
        Ok(AccessToken::new(token))
    }
    fn generate_refresh_token(&self) -> RepoResult<RefreshToken> {
        Ok(RefreshToken::new(Uuid::new_v4().to_string()))
    }
    fn verify_access_token(&self, token: &str) -> RepoResult<bool> {
        Ok(decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(&self.secret.as_bytes()),
            &Default::default(),
        )
        .is_ok())
    }
}
