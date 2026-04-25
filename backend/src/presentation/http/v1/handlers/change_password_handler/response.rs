use serde::Serialize;
use utoipa::ToSchema;

use crate::application::results::change_password_result::ChangePasswordResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct ChangePasswordResponseData;

impl From<ChangePasswordResult> for ChangePasswordResponseData {
    fn from(_result: ChangePasswordResult) -> ChangePasswordResponseData {
        Self {}
    }
}
