use crate::domain::auth::entities::access_token::AccessToken;
use crate::domain::auth::entities::refresh_token::RefreshToken;
use crate::domain::auth::value_objects::token_id::TokenId;
use crate::domain::identities::value_objects::user::user_id::UserId;

pub struct TokenPair {
    id: TokenId,
    user_id: UserId,
    access_token: AccessToken,
    refresh_token: RefreshToken,
}
