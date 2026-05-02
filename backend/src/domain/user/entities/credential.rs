use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::value_objects::credential::credential_id::CredentialId;
use crate::domain::user::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::user::value_objects::credential::credential_status::CredentialStatus;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    id: CredentialId,
    kind: CredentialKind,
    status: CredentialStatus,
    created_at: Timestamp,
    last_login_at: Option<Timestamp>,
}

impl Credential {
    pub fn new(credential_kind: &CredentialKind, credential_status: &CredentialStatus) -> Self {
        let credential_id = CredentialId::new();
        let created_at = Timestamp::now();
        Credential {
            id: credential_id,
            kind: credential_kind.to_owned(),
            status: credential_status.to_owned(),
            created_at,
            last_login_at: None,
        }
    }
    pub fn id(&self) -> &CredentialId {
        &self.id
    }
    pub fn kind(&self) -> &CredentialKind {
        &self.kind
    }
    pub fn status(&self) -> &CredentialStatus {
        &self.status
    }
    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
    pub fn last_login_at(&self) -> &Option<Timestamp> {
        &self.last_login_at
    }
    pub fn update_password_credential(
        &mut self,
        new_password_credential: &PasswordCredential,
    ) -> bool {
        match &mut self.kind {
            CredentialKind::Password(password_credential) => {
                password_credential.update_password_credential(new_password_credential);
                true
            } 
        }
    }
}
