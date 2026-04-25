use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;

pub trait RefreshTokenService {
    fn generate(&self) -> RefreshToken;
}
