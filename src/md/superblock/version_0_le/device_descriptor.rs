use binary_layout::prelude::*;

pub use layout::NestedView as NestedDeviceDescriptor;
pub use layout::View as DeviceDescriptor;

define_layout!(layout, LittleEndian, {
    number: u32,
    major: u32,
    minor: u32,
    raid_disk: u32,
    state: u32,
    reserved: [u8; 108]
});

impl<S: AsRef<[u8]>> DeviceDescriptor<S> {
    pub const SIZE: usize = match layout::SIZE {
        Some(x) => x,
        None => panic!(),
    };
}