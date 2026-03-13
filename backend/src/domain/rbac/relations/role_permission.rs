use crate::domain::rbac::entities::permission::permission_id::PermissionId;
use crate::domain::rbac::entities::role::role_id::RoleId;

pub struct RolePermission {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}
