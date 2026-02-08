use crate::domain::auth::value_objects::user_email::UserEmail;

pub struct SendEmailCommand {
    pub user_email: UserEmail,
}
