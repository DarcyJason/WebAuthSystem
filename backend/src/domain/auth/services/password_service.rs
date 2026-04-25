use crate::domain::error::DomainResult;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;

pub trait PasswordService: Send + Sync {
    fn hash_password(&self, plain_password: PlainPassword) -> DomainResult<PasswordCredential>;
    fn verify_password(
        &self,
        hashed_password: PasswordCredential,
        plain_password: PlainPassword,
    ) -> DomainResult<bool>;
}
