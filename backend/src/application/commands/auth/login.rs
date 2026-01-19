use serde::Deserialize;

use crate::domain::auth::value_objects::{
    login_identity::LoginIdentity, plain_password::PlainPassword,
};

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub identity: LoginIdentity,
    pub password: PlainPassword,
}
