use crate::domain::rbac::value_objects::permission_code::PermissionCode;

pub enum RbacError {
    PermissionDenied,
}

pub struct RbacPolicy;

impl RbacPolicy {
    pub fn check(
        user_permissions: &[PermissionCode],
        required: &PermissionCode,
    ) -> Result<(), RbacError> {
        if user_permissions.contains(required) {
            Ok(())
        } else {
            Err(RbacError::PermissionDenied)
        }
    }
}
