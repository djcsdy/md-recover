use binary_layout::binary_layout;

#[allow(unused_imports)]
pub use self::layout::NestedView as NestedExtentHeader;

binary_layout!(layout, LittleEndian, {
    magic: u16,
    entries: u16,
    max: u16,
    depth: u16,
    generation: u32,
});
