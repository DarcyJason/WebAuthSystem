use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialKind {
    Password(PasswordCredential),
    // OAuth(OAuthCredential),
    // Passkey(PasskeyCredential),
}

impl CredentialKind {
    pub fn new(password_credential: &PasswordCredential) -> Self {
        CredentialKind::Password(password_credential.to_owned())
    }
}
