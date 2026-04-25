use crate::application::commands::reset_password_command::ResetPasswordCommand;
use crate::application::error::{ApplicationError, PasswordsNotMatchedSnafu, ValidationSnafu};
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequestPayload {
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}

impl TryInto<ResetPasswordCommand> for ResetPasswordRequestPayload {
    type Error = ApplicationError;
    fn try_into(self) -> Result<ResetPasswordCommand, Self::Error> {
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
        Ok(ResetPasswordCommand {
            token: self.token,
            new_password,
        })
    }
}
