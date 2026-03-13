use crate::domain::{
    auth::value_objects::tokens::verification_token::VerificationToken,
    user::entities::user::user_email::UserEmail,
};

pub struct ValidateVerificationCommand {
    pub email: UserEmail,
    pub verification_token: VerificationToken,
}
