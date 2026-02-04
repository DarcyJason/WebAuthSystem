use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequestPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
