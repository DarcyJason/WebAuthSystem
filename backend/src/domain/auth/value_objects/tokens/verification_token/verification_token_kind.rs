use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_token_kind", rename_all = "PascalCase")]
pub enum VerificationTokenKind {
    EmailVerification,
    PasswordReset,
}
