use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::results::resend_verification_result::ResendVerificationResult;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationResponseData;

impl From<ResendVerificationResult> for ResendVerificationResponseData {
    fn from(_result: ResendVerificationResult) -> Self {
        Self {}
    }
}
