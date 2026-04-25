use serde::Serialize;
use utoipa::ToSchema;

use crate::application::results::verify_result::VerifyResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponseData;

impl From<VerifyResult> for VerifyResponseData {
    fn from(_: VerifyResult) -> Self {
        Self {}
    }
}
