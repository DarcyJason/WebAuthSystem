use crate::application::commands::register_command::RegisterCommand;
use crate::application::error::{ApplicationError, PasswordsNotMatchedSnafu, ValidationSnafu};
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_name::UserName;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequestPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl TryInto<RegisterCommand> for RegisterRequestPayload {
    type Error = ApplicationError;
    fn try_into(self) -> Result<RegisterCommand, Self::Error> {
        let name = UserName::new(self.name).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        let email = UserEmail::new(self.email).map_err(|e| {
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
        let plain_confirm_password = PlainPassword::new(self.confirm_password).map_err(|e| {
            ValidationSnafu {
                message: e.to_string(),
            }
            .build()
        })?;
        if plain_password != plain_confirm_password {
            return PasswordsNotMatchedSnafu {}.fail();
        }
        Ok(RegisterCommand {
            name,
            email,
            plain_password,
        })
    }
}
