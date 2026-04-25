use crate::domain::auth::value_objects::tokens::access_token::AccessToken;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;

pub struct RotateRefreshTokenResult {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
