use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    checksum: u32,
});
