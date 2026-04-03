use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_name::UserName;

pub enum UserNameOrUserEmail {
    UserName(UserName),
    UserEmail(UserEmail),
}
