use crate::application::commands::register_command::RegisterCommand;
use crate::application::error::{ApplicationResult, FieldInvalidSnafu};
use crate::domain::identities::value_objects::credential::plain_password::PlainPassword;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_name::UserName;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

impl RegisterRequest {
    pub fn into_command(self) -> ApplicationResult<RegisterCommand> {
        Ok(RegisterCommand {
            name: UserName::new(self.name).map_err(|e| {
                FieldInvalidSnafu {
                    message: e.to_string(),
                }
                .build()
            })?,
            email: UserEmail::new(self.email).map_err(|e| {
                FieldInvalidSnafu {
                    message: e.to_string(),
                }
                .build()
            })?,
            plain_password: PlainPassword::new(self.password).map_err(|e| {
                FieldInvalidSnafu {
                    message: e.to_string(),
                }
                .build()
            })?,
        })
    }
}
