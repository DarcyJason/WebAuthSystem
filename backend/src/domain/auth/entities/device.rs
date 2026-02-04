use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::domain::{
    auth::value_objects::{device_id::DeviceId, device_name::DeviceName},
    common::time::timestamp::Timestamp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    id: DeviceId,
    name: DeviceName,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl Device {
    pub fn new(device_name: DeviceName) -> Self {
        let device_id = DeviceId::new();
        let created_at = Timestamp::new(Utc::now());
        let updated_at = created_at.clone();
        Device {
            id: device_id,
            name: device_name,
            created_at,
            updated_at,
        }
    }
}
