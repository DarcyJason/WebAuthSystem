use crate::domain::auth::entity::device::Device;
use crate::domain::auth::entity::user::User;

pub struct UserAggregate {
    pub user: User,
    pub devices: Vec<Device>,
}
