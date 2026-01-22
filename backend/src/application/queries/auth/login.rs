use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::username::Username;
use surrealdb::RecordId;

#[derive(Debug, Clone)]
pub struct LoginResult {
    pub user_id: RecordId,
    pub username: Username,
    pub email: Email,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl LoginResult {
    pub fn from(user: User, access_token: AccessToken, refresh_token: RefreshToken) -> Self {
        LoginResult {
            user_id: user.id().to_owned(),
            username: user.username().to_owned(),
            email: user.email().to_owned(),
            access_token,
            refresh_token,
        }
    }
}
