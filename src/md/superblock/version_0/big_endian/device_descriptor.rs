use binary_layout::prelude::*;

pub struct DeviceDescriptor<S: AsRef<[u8]>>(layout::View<S>);

binary_layout!(layout, BigEndian, {
    number: u32,
    major: u32,
    minor: u32,
    role: u32,
    state: u32,
    reserved: [u8; 108]
});

impl<S: AsRef<[u8]>> DeviceDescriptor<S> {
    pub const SIZE: usize = match layout::SIZE {
        Some(size) => size,
        None => panic!(),
    };

    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn number(&self) -> u32 {
        self.0.number().read()
    }

    pub fn major(&self) -> u32 {
        self.0.major().read()
    }

    pub fn minor(&self) -> u32 {
        self.0.minor().read()
    }

    pub fn role(&self) -> u32 {
        self.0.role().read()
    }

    pub fn state(&self) -> u32 {
        self.0.state().read()
    }
}
