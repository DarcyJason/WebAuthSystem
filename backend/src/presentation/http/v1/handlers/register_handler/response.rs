use crate::application::results::register_result::RegisterResult;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseData {
    pub user_id: String,
}

impl From<&RegisterResult> for RegisterResponseData {
    fn from(result: &RegisterResult) -> Self {
        RegisterResponseData {
            user_id: result.user_id.value().to_string(),
        }
    }
}
