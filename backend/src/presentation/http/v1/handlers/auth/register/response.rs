use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponseData {
    pub user_id: String,
    pub username: String,
    pub email: String,
}
