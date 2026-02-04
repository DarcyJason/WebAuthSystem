use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequestPayload {
    pub username_or_email: String,
    pub password: String,
}
