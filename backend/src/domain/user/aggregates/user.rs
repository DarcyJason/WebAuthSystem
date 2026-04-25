use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::entities::credential::Credential;
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::domain::user::value_objects::user::user_name::UserName;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserEntity {
    id: UserId,
    name: UserName,
    email: UserEmail,
    #[sqlx(json)]
    credentials: Vec<Credential>,
    status: UserStatus,
    access_token_version: AccessTokenVersion,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl UserEntity {
    pub fn new(user_name: UserName, user_email: UserEmail, credentials: Vec<Credential>) -> Self {
        let user_id = UserId::new();
        let created_at = Timestamp::now();
        let updated_at = created_at.clone();
        let access_token_version = AccessTokenVersion::new();
        UserEntity {
            id: user_id,
            name: user_name,
            email: user_email,
            credentials,
            status: UserStatus::EmailNotVerified,
            access_token_version,
            created_at,
            updated_at,
        }
    }
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn name(&self) -> &UserName {
        &self.name
    }
    pub fn email(&self) -> &UserEmail {
        &self.email
    }
    pub fn credentials(&self) -> &Vec<Credential> {
        &self.credentials
    }
    pub fn status(&self) -> &UserStatus {
        &self.status
    }
    pub fn access_token_version(&self) -> &AccessTokenVersion {
        &self.access_token_version
    }
    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
    pub fn updated_at(&self) -> &Timestamp {
        &self.updated_at
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
    pub fn update_access_token_version(&mut self, access_token_version: &AccessTokenVersion) {
        self.access_token_version = access_token_version.to_owned();
    }
}
