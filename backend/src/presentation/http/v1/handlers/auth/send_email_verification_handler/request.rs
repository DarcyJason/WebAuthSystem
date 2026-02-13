use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendEmailVerificationRequestPayload {
    pub email: String,
}
