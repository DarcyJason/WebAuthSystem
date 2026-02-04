use serde::{Deserialize, Serialize};

use crate::domain::rbac::value_objects::permission_code::PermissionCode;
use crate::domain::rbac::value_objects::role_id::RoleId;
use crate::domain::rbac::value_objects::role_name::RoleName;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    pub permission: Vec<PermissionCode>,
}

impl Role {
    pub fn new(role_name: RoleName, role_permission: Vec<PermissionCode>) -> Self {
        let role_id = RoleId::new();
        Role {
            id: role_id,
            name: role_name,
            permission: role_permission,
        }
    }
}
