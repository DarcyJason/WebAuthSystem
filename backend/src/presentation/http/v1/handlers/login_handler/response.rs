use crate::application::results::login_result::LoginResult;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginResponseData;

impl From<&LoginResult> for LoginResponseData {
    fn from(_result: &LoginResult) -> Self {
        LoginResponseData
    }
}
