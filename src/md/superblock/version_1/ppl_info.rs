use binary_layout::prelude::*;

pub use layout::View as PplInfo;

binary_layout!(layout, LittleEndian, {
    offset: i16,
    size: u16,
});

impl<B: AsRef<[u8]>> PplInfo<B> {
    pub const LENGTH: usize = 4;
}
