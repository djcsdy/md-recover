use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum FileType {
    Socket = 12,
    SymbolicLink = 10,
    RegularFile = 8,
    BlockDevice = 6,
    Directory = 4,
    CharacterDevice = 2,
    Fifo = 1,
    #[num_enum(catch_all)]
    Unknown(u8) = 0,
}
