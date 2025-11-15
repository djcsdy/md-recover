use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    version: u32,
});

pub use layout::NestedView as NestedLinuxSpecific1;
