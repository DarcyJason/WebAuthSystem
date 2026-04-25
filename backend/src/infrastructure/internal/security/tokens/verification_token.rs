use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::user::value_objects::user::user_id::UserId;

#[derive(Debug, Clone)]
pub struct DefaultVerificationTokenService {
    pub email_verify_expires_in_seconds: i64,
    pub password_reset_expires_in_seconds: i64,
}

impl DefaultVerificationTokenService {
    pub fn new(
        email_verify_expires_in_seconds: i64,
        password_reset_expires_in_seconds: i64,
    ) -> Self {
        Self {
            email_verify_expires_in_seconds,
            password_reset_expires_in_seconds,
        }
    }

    pub fn issue_email_verification(&self, user_id: UserId) -> VerificationToken {
        VerificationToken::issue(
            user_id,
            VerificationTokenKind::EmailVerification,
            self.email_verify_expires_in_seconds,
        )
    }

    pub fn issue_password_reset(&self, user_id: UserId) -> VerificationToken {
        VerificationToken::issue(
            user_id,
            VerificationTokenKind::PasswordReset,
            self.password_reset_expires_in_seconds,
        )
    }
}
