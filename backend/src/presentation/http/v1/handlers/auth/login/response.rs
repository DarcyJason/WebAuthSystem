use serde::Serialize;
use utoipa::ToSchema;

use crate::application::queries::auth::login::LoginResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

impl From<LoginResult> for LoginResponse {
    fn from(result: LoginResult) -> Self {
        LoginResponse {
            user_id: result.user_id,
            username: result.username,
            email: result.email,
        }
    }
}
