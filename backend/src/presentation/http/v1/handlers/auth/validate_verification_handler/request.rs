use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ValidateEmailVerificationRequestPayload {
    pub email: String,
    pub verification_token: String,
}
