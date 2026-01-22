use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponseData {
    pub user_id: String,
    pub username: String,
    pub email: String,
}
