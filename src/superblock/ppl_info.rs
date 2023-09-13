use binary_layout::prelude::*;

define_layout!(layout, LittleEndian, {
    offset: i16,
    size: u16,
});

pub use layout::View as PplInfo;

impl<B: AsRef<[u8]>> PplInfo<B> {
    pub const LENGTH: usize = 4;
}
