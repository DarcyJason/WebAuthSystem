use crate::domain::rbac::value_objects::permission::permission_id::PermissionId;
use crate::domain::rbac::value_objects::role::role_id::RoleId;

pub struct RolePermission {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}
