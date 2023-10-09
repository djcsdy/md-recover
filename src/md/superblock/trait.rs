pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
}