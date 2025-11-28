use iocuddle::{Group, Ioctl, Read};

pub const BLK: Group = Group::new(0x12);

pub const BLK_GETSIZE64: Ioctl<Read, &u64> = unsafe { BLK.read(114) };
pub const BLK_PBSZGET: Ioctl<Read, &u32> = unsafe { BLK.read(123) };
