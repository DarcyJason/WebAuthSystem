use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::identities::entities::credential::Credential;
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::user::user_name::UserName;
use crate::domain::identities::value_objects::user::user_status::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
    #[sqlx(json)]
    credentials: Vec<Credential>,
    status: UserStatus,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl User {
    pub fn new(user_name: UserName, user_email: UserEmail, credentials: Vec<Credential>) -> Self {
        let user_id = UserId::new();
        let created_at = Timestamp::now();
        let updated_at = created_at.clone();
        User {
            id: user_id,
            name: user_name,
            email: user_email,
            credentials,
            status: UserStatus::EmailNotVerified,
            created_at,
            updated_at,
        }
    }
    pub fn id(&self) -> UserId {
        self.id.to_owned()
    }
    pub fn name(&self) -> UserName {
        self.name.to_owned()
    }
    pub fn email(&self) -> UserEmail {
        self.email.to_owned()
    }
    pub fn credentials(&self) -> Vec<Credential> {
        self.credentials.to_owned()
    }
    pub fn status(&self) -> UserStatus {
        self.status.to_owned()
    }
    pub fn created_at(&self) -> Timestamp {
        self.created_at.to_owned()
    }
    pub fn updated_at(&self) -> Timestamp {
        self.updated_at.to_owned()
    }
    pub fn update_name(&mut self, user_name: &UserName) {
        self.name.update_name(user_name)
    }
    pub fn update_status(&mut self, user_status: &UserStatus) {
        self.status.update_status(user_status)
    }
    pub fn update_password_credential(&mut self, new_password_credential: &PasswordCredential) {
        for credential in &mut self.credentials {
            if credential.update_password_credential(new_password_credential) {
                break;
            }
        }
    }
}
