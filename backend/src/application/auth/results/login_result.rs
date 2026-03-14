use crate::domain::auth::value_objects::tokens::{
    access_token::AccessToken, refresh_token::RefreshToken,
};
use crate::domain::auth::value_objects::user::user_email::UserEmail;

#[derive(Debug, Clone)]
pub struct LoginResult {
    pub user_email: UserEmail,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
