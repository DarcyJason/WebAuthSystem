use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::user::user_email::UserEmail;

pub struct ValidateVerificationCommand {
    pub email: UserEmail,
    pub verification_token: VerificationToken,
}
