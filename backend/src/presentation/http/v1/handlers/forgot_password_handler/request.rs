use crate::application::error::{ApplicationError, ValidationSnafu};
use crate::domain::user::value_objects::user::user_email::UserEmail;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPasswordRequestPayload {
    pub email: String,
}

impl TryInto<UserEmail> for ForgotPasswordRequestPayload {
    type Error = ApplicationError;

    fn try_into(self) -> Result<UserEmail, Self::Error> {
        UserEmail::new(self.email).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })
    }
}
