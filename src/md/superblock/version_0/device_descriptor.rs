pub struct DeviceDescriptor {
    pub number: u32,
    pub major: u32,
    pub minor: u32,
    pub role: u32,
    pub state: u32,
}

impl DeviceDescriptor {
    const ROLE_SPARE: u32 = 0xffff;
    const ROLE_FAULTY: u32 = 0xfffe;
    const ROLE_JOURNAL: u32 = 0xfffd;
    const ROLE_MAX: u32 = 0xfeff;

    pub fn is_valid(&self) -> bool {
        !(Self::ROLE_MAX..Self::ROLE_JOURNAL).contains(&self.role)
    }
}
