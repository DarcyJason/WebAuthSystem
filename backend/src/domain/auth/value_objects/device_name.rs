use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceName(String);

impl DeviceName {
    pub fn new(device_name: String) -> Self {
        DeviceName(device_name)
    }
}
