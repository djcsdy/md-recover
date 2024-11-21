const ROLE_SPARE: u32 = 0xffff;
const ROLE_FAULTY: u32 = 0xfffe;
const ROLE_JOURNAL: u32 = 0xfffd;
const ROLE_MAX: u32 = 0xfeff;

pub trait DeviceDescriptor {
    fn is_valid(&self) -> bool {
        let role = self.role();
        !(ROLE_MAX..ROLE_JOURNAL).contains(&role)
    }

    fn number(&self) -> u32;
    fn major(&self) -> u32;
    fn minor(&self) -> u32;
    fn role(&self) -> u32;
    fn state(&self) -> u32;
}

impl DeviceDescriptor for Box<dyn DeviceDescriptor> {
    fn number(&self) -> u32 {
        self.as_ref().number()
    }

    fn major(&self) -> u32 {
        self.as_ref().major()
    }

    fn minor(&self) -> u32 {
        self.as_ref().minor()
    }

    fn role(&self) -> u32 {
        self.as_ref().role()
    }

    fn state(&self) -> u32 {
        self.as_ref().state()
    }
}
