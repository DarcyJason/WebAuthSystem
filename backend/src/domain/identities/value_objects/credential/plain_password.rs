use crate::domain::error::{DomainResult, InvalidPlainPassowrdSnafu};
use snafu::ensure;

#[derive(Debug, Clone)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(plain_password: impl Into<String>) -> DomainResult<Self> {
        let plain_password = plain_password.into();
        ensure!(
            plain_password.len() != 0,
            InvalidPlainPassowrdSnafu {
                password: plain_password.clone(),
                message: "password is required".to_string()
            }
        );
        ensure!(
            plain_password.len() > 8,
            InvalidPlainPassowrdSnafu {
                password: plain_password.clone(),
                message: "password is too short".to_string()
            }
        );
        ensure!(
            plain_password.len() < 20,
            InvalidPlainPassowrdSnafu {
                password: plain_password,
                message: "password is too long".to_string()
            }
        );
        Ok(Self(plain_password.into()))
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
