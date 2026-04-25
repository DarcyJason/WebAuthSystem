use crate::application::commands::login_command::LoginCommand;
use crate::application::error::{ApplicationError, ValidationSnafu};
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use crate::domain::user::value_objects::user::user_name_or_user_email::UserNameOrUserEmail;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestPayload {
    pub name_or_email: String,
    pub password: String,
}

impl TryInto<LoginCommand> for LoginRequestPayload {
    type Error = ApplicationError;
    fn try_into(self) -> Result<LoginCommand, Self::Error> {
        let name_or_email = UserNameOrUserEmail::new(self.name_or_email).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        let plain_password = PlainPassword::new(self.password).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        Ok(LoginCommand {
            name_or_email,
            plain_password,
        })
    }
}
