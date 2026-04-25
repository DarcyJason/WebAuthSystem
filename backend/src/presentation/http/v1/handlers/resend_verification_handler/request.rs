use crate::application::commands::resend_verification_command::ResendVerificationCommand;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::presentation::http::v1::error::ApiError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationRequestPayload {
    pub email: String,
}

impl TryFrom<ResendVerificationRequestPayload> for ResendVerificationCommand {
    type Error = ApiError;

    fn try_from(payload: ResendVerificationRequestPayload) -> Result<Self, Self::Error> {
        Ok(ResendVerificationCommand {
            email: UserEmail::new(payload.email).map_err(|e| ApiError::BadRequest {
                message: e.to_string(),
            })?,
        })
    }
}
