use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "credential_status", rename_all = "PascalCase")]
pub enum CredentialStatus {
    Active,
    Revoked,
}
