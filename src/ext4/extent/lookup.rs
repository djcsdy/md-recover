use crate::ext4::units::FsBlockIndex;

pub enum ExtentLookup {
    OutOfBounds,
    Indirect(FsBlockIndex),
    Direct(FsBlockIndex),
}
