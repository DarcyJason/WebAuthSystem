use serde::Serialize;
use utoipa::ToSchema;

use crate::application::queries::auth::register::RegisterResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

impl From<RegisterResult> for RegisterResponse {
    fn from(result: RegisterResult) -> Self {
        RegisterResponse {
            user_id: result.user_id,
            username: result.username,
            email: result.email,
        }
    }
}
