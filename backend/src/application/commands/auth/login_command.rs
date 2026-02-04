use crate::domain::auth::value_objects::{
    login_identity::LoginIdentity, plain_password::PlainPassword,
};

pub struct LoginCommand {
    pub login_identity: LoginIdentity,
    pub plain_password: PlainPassword,
}
