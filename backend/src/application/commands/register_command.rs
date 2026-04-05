use crate::domain::identities::value_objects::credential::plain_password::PlainPassword;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_name::UserName;

pub struct RegisterCommand {
    pub name: UserName,
    pub email: UserEmail,
    pub plain_password: PlainPassword,
}
