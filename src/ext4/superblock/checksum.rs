#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Checksum {
    None,
    Crc32c(u32),
    Unknown(u8, u32),
}
