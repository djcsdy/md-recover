use binary_layout::define_layout;

define_layout!(layout, LittleEndian, {
    reserved_zero_1: u32,
    record_length: u16,
    reserved_zero_2: u8,
    reserved_file_type: u8,
    checksum: u32
});

pub struct DirEntryTail<S: AsRef<[u8]>>(layout::View<S>);
