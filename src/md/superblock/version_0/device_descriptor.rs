pub trait DeviceDescriptor {
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