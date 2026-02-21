use crate::domain::auth::value_objects::{
    user_email::UserEmail, verification_token::VerificationToken,
};

pub struct ValidateEmailVerificationCommand {
    pub email: UserEmail,
    pub verification_token: VerificationToken,
}
