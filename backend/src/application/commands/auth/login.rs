use serde::Deserialize;

use crate::domain::auth::value_objects::{LoginIdentity, PlainPassword};

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub identity: LoginIdentity,
    pub password: PlainPassword,
}
