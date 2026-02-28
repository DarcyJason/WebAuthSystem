use crate::domain::{
    auth::value_objects::verification_token::VerificationToken,
    user::value_objects::user_email::UserEmail,
};

pub struct ValidateVerificationCommand {
    pub email: UserEmail,
    pub verification_token: VerificationToken,
}
