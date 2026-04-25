use crate::application::results::rotate_refresh_token_result::RotateRefreshTokenResult;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct RotateRefreshTokenResponseData {
    pub token_type: String,
}

impl From<&RotateRefreshTokenResult> for RotateRefreshTokenResponseData {
    fn from(_result: &RotateRefreshTokenResult) -> Self {
        Self {
            token_type: "Bearer".to_string(),
        }
    }
}
