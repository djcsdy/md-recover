use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use binary_layout::prelude::*;

pub struct DeviceDescriptorLittleEndian<S: AsRef<[u8]>>(layout::View<S>);

binary_layout!(layout, LittleEndian, {
    number: u32,
    major: u32,
    minor: u32,
    role: u32,
    state: u32,
    reserved: [u8; 108]
});

impl<S: AsRef<[u8]>> DeviceDescriptorLittleEndian<S> {
    pub const SIZE: usize = match layout::SIZE {
        Some(size) => size,
        None => panic!(),
    };

    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }
}

impl<S: AsRef<[u8]>> DeviceDescriptor for DeviceDescriptorLittleEndian<S> {
    fn number(&self) -> u32 {
        self.0.number().read()
    }

    fn major(&self) -> u32 {
        self.0.major().read()
    }

    fn minor(&self) -> u32 {
        self.0.minor().read()
    }

    fn role(&self) -> u32 {
        self.0.role().read()
    }

    fn state(&self) -> u32 {
        self.0.state().read()
    }
}
