use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialKind {
    Password(PasswordCredential),
    
    
}

impl CredentialKind {
    pub fn new(password_credential: &PasswordCredential) -> Self {
        CredentialKind::Password(password_credential.to_owned())
    }
    pub fn password_credential(&self) -> Option<&PasswordCredential> {
        match self {
            CredentialKind::Password(password_credential) => Some(password_credential),
        }
    }
}
