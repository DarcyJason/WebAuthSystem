use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetMeResponseData {
    pub name: String,
    pub email: String,
    pub created_at: String,
}
