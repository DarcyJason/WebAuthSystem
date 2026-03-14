use crate::domain::common::time::timestamp::Timestamp;
use crate::domain::rbac::value_objects::role::role_id::RoleId;
use crate::domain::rbac::value_objects::role::role_name::RoleName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Role {
    pub fn new(role_name: RoleName) -> Self {
        let role_id = RoleId::new();
        let now = Timestamp::now();
        let created_at = now.clone();
        let updated_at = now;
        Role {
            id: role_id,
            name: role_name,
            created_at,
            updated_at,
        }
    }
}
