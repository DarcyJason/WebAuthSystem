use crate::domain::error::{DomainResult, InvalidUserNameOrUserEmailSnafu};
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_name::UserName;
use snafu::ensure;

pub enum UserNameOrUserEmail {
    UserName(UserName),
    UserEmail(UserEmail),
}

impl UserNameOrUserEmail {
    pub fn new(user_name_or_user_email: impl Into<String>) -> DomainResult<UserNameOrUserEmail> {
        let user_name_or_user_email = user_name_or_user_email.into();
        ensure!(
            !user_name_or_user_email.is_empty(),
            InvalidUserNameOrUserEmailSnafu {
                message: "user_name_or_user_email required".to_string()
            }
        );
        ensure!(
            user_name_or_user_email.len() < 100,
            InvalidUserNameOrUserEmailSnafu {
                message: "user_name_or_user_email must be less than 100 characters".to_string()
            }
        );
        if user_name_or_user_email.contains('@') {
            Ok(UserNameOrUserEmail::UserEmail(
                UserEmail::new(user_name_or_user_email).map_err(|e| {
                    InvalidUserNameOrUserEmailSnafu {
                        message: e.to_string(),
                    }
                    .build()
                })?,
            ))
        } else {
            Ok(UserNameOrUserEmail::UserName(
                UserName::new(user_name_or_user_email).map_err(|e| {
                    InvalidUserNameOrUserEmailSnafu {
                        message: e.to_string(),
                    }
                    .build()
                })?,
            ))
        }
    }
}
