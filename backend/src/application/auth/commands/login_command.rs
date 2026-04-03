use crate::domain::identities::value_objects::credential::credential_kind::CredentialKind;
use crate::domain::identities::value_objects::user::user_name_or_user_email::UserNameOrUserEmail;

pub struct LoginCommand {
    pub user_name_or_user_email: UserNameOrUserEmail,
    pub credential_kind: CredentialKind,
}
