use serde::Serialize;
use utoipa::ToSchema;

use crate::application::results::logout_result::LogoutResult;

#[derive(Serialize, ToSchema)]
pub struct LogoutResponseData {}

impl From<LogoutResult> for LogoutResponseData {
    fn from(_result: LogoutResult) -> Self {
        Self {}
    }
}
