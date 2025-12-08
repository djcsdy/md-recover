use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use crate::md::superblock::MdDeviceRole;
use binary_layout::prelude::*;

pub use layout::View as DeviceDescriptorLittleEndian;

pub use layout::NestedView as NestedDeviceDescriptorLittleEndian;

binary_layout!(layout, LittleEndian, {
    index: u32,
    major: u32,
    minor: u32,
    role: MdDeviceRole as u32,
    state: u32,
    reserved: [u8; 108]
});

impl<S: AsRef<[u8]>> DeviceDescriptorLittleEndian<S> {
    pub const SIZE: usize = layout::SIZE.unwrap();
}

impl<S: AsRef<[u8]>> From<DeviceDescriptorLittleEndian<S>> for DeviceDescriptor {
    fn from(value: DeviceDescriptorLittleEndian<S>) -> Self {
        Self {
            index: value.index().read(),
            major: value.major().read(),
            minor: value.minor().read(),
            role: value.role().read(),
            state: value.state().read(),
        }
    }
}
