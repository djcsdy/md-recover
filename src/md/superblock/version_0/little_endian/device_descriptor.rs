use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use binary_layout::prelude::*;

pub use layout::View as DeviceDescriptorLittleEndian;

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
}

impl<S: AsRef<[u8]>> From<DeviceDescriptorLittleEndian<S>> for DeviceDescriptor {
    fn from(value: DeviceDescriptorLittleEndian<S>) -> Self {
        Self {
            number: value.number().read(),
            major: value.major().read(),
            minor: value.minor().read(),
            role: value.role().read(),
            state: value.state().read(),
        }
    }
}
