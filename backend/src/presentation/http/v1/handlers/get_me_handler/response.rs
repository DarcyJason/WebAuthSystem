use crate::presentation::http::v1::middlewares::auth::AuthMiddleware;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponseData {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub status: String,
}

impl From<&AuthMiddleware> for GetMeResponseData {
    fn from(auth: &AuthMiddleware) -> Self {
        Self {
            user_id: auth.user.id().value().to_string(),
            name: auth.user.name().value().to_string(),
            email: auth.user.email().value().to_string(),
            status: auth.user.status().as_str().to_string(),
        }
    }
}
