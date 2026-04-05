use crate::domain::identities::value_objects::credential::plain_password::PlainPassword;
use crate::domain::identities::value_objects::user::user_name_or_user_email::UserNameOrUserEmail;

pub struct LoginCommand {
    pub name_or_email: UserNameOrUserEmail,
    pub plain_password: PlainPassword,
}
