use binary_layout::prelude::*;

pub use self::layout::NestedView;

define_layout!(layout, BigEndian, {
    number: u32,
    major: u32,
    minor: u32,
    raid_disk: u32,
    state: u32,
    reserved: [u8; 108]
});

pub const SIZE: usize = match layout::SIZE {
    Some(size) => size,
    None => panic!(),
};
