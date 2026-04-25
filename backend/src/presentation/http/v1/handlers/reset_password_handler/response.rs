use serde::Serialize;
use utoipa::ToSchema;

use crate::application::results::reset_password_result::ResetPasswordResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordResponseData;

impl From<ResetPasswordResult> for ResetPasswordResponseData {
    fn from(_result: ResetPasswordResult) -> Self {
        Self {}
    }
}
