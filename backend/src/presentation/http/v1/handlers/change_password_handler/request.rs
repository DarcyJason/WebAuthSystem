use crate::application::error::{ApplicationError, PasswordsNotMatchedSnafu, ValidationSnafu};
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequestPayload {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

impl ChangePasswordRequestPayload {
    pub fn validate_passwords(&self) -> Result<(), ApplicationError> {
        let _current_password = PlainPassword::new(self.current_password.clone()).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        let new_password = PlainPassword::new(self.new_password.clone()).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        let confirm_password = PlainPassword::new(self.confirm_password.clone()).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        if new_password != confirm_password {
            return PasswordsNotMatchedSnafu {}.fail();
        }
        Ok(())
    }
}
