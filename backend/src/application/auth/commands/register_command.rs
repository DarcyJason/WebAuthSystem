use crate::domain::identities::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_name::UserName;

pub struct RegisterCommand {
    pub name: UserName,
    pub email: UserEmail,
    pub credential_kind: CredentialKind,
}
