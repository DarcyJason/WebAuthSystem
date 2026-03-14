use crate::domain::common::time::timestamp::Timestamp;
use crate::domain::rbac::value_objects::permission::permission_code::PermissionCode;
use crate::domain::rbac::value_objects::permission::permission_description::PermissionDescription;
use crate::domain::rbac::value_objects::permission::permission_id::PermissionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: PermissionId,
    pub code: PermissionCode,
    pub description: PermissionDescription,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Permission {
    pub fn new(
        permission_code: PermissionCode,
        permission_description: PermissionDescription,
    ) -> Self {
        let permission_id = PermissionId::new();
        let now = Timestamp::now();
        let created_at = now.clone();
        let updated_at = now;
        Permission {
            id: permission_id,
            code: permission_code,
            description: permission_description,
            created_at,
            updated_at,
        }
    }
}
