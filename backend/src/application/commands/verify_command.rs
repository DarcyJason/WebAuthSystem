use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;

pub struct VerifyCommand {
    pub token_value: VerificationTokenValue,
}
