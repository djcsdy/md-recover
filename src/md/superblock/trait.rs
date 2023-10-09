use std::ffi::OsStr;
use super::ArrayUuid;

pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
    fn array_uuid(&self) -> ArrayUuid;
    fn array_name(&self) -> Option<&OsStr>;
}