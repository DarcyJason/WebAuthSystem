use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    application::{commands::verify_command::VerifyCommand, error::ApplicationError},
    domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue,
};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VerifyRequestPayload {
    pub token: String,
}

impl TryInto<VerifyCommand> for VerifyRequestPayload {
    type Error = ApplicationError;

    fn try_into(self) -> Result<VerifyCommand, Self::Error> {
        Ok(VerifyCommand {
            token: VerificationTokenValue::from(self.token),
        })
    }
}
