bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
    pub struct Flags : u16 {
        const INODE_NOT_IN_USE = 1;
        const BLOCK_BITMAP_NOT_IN_USE = 2;
        const INODE_TABLE_ZEROED = 4;
    }
}
