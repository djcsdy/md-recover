use crate::md::superblock::MdDeviceRole;

pub struct DeviceDescriptor {
    pub index: u32,
    pub major: u32,
    pub minor: u32,
    pub role: MdDeviceRole,
    pub state: u32,
}

impl DeviceDescriptor {
    pub fn is_valid(&self) -> bool {
        self.role.is_valid()
    }
}
