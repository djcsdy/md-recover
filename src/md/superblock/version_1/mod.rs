use std::io::{Error, ErrorKind, Read, Result};

use binary_layout::prelude::*;
use byteorder::{ByteOrder, LittleEndian};

use device_flags::DeviceFlags;
use features::Features;
use ppl_info::PplInfo;
use reshape_info::NestedReshapeInfo;

use crate::md::superblock::{ArrayUuid, Superblock};

mod device_flags;
mod features;
mod ppl_info;
mod reshape_info;

define_layout!(layout, LittleEndian, {
    magic: u32,
    major_version: u32,
    features: Features as u32,
    pad_0: u32,
    array_uuid: [u8; 16],
    array_name: [u8; 32],
    ctime: u64,
    level: u32,
    layout: u32,
    size: u64,
    chunk_size: u32,
    raid_disks: u32,
    bitmap_offset_or_ppl_info: [u8; 4],
    reshape_info: NestedReshapeInfo,
    data_offset: u64,
    data_size: u64,
    super_offset: u64,
    recovery_offset_or_journal_tail: u64,
    device_number: u32,
    count_corrected_read: u32,
    device_uuid: [u8; 16],
    device_flags: DeviceFlags as u8,
    bad_block_log_shift: u8,
    bad_block_log_size: u16,
    bad_block_log_offset: u32,
    utime: u64,
    events: u64,
    resync_offset: u64,
    superblock_checksum: u32,
    max_devices: u32,
    pad_3: [u8; 32],
    dev_roles: [u8]
});

pub struct SuperblockVersion1<S: AsRef<[u8]>>(layout::View<S>);

impl SuperblockVersion1<Vec<u8>> {
    pub const MAX_SIZE: usize = 4096;

    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = vec![0u8; Self::MAX_SIZE];
        reader.read_exact(&mut buf)?;
        let superblock = Self::new(buf);
        if superblock.valid() {
            Ok(superblock)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}

impl<S: AsRef<[u8]>> SuperblockVersion1<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn valid_magic(&self) -> bool {
        self.0.magic().read() == 0xa92b4efc
    }

    pub fn valid_major_version(&self) -> bool {
        self.major_version() == 1
    }

    fn features(&self) -> Features {
        self.0.features().read()
    }

    fn has_bitmap_offset(&self) -> bool {
        self.features().contains(Features::BITMAP_OFFSET)
    }

    fn has_recovery_offset(&self) -> bool {
        self.features().contains(Features::RECOVERY_OFFSET)
    }

    fn has_journal(&self) -> bool {
        self.features().contains(Features::JOURNAL)
    }

    fn has_ppl(&self) -> bool {
        self.features().contains(Features::PPL)
    }

    pub fn array_name(&self) -> &[u8; 32] {
        self.0.array_name()
    }

    pub fn bitmap_offset(&self) -> Option<u32> {
        if self.has_bitmap_offset() {
            Some(LittleEndian::read_u32(self.0.bitmap_offset_or_ppl_info()))
        } else {
            None
        }
    }

    pub fn ppl_info(&self) -> Option<PplInfo<&[u8]>> {
        if self.has_ppl() {
            Some(PplInfo::new(self.0.bitmap_offset_or_ppl_info()))
        } else {
            None
        }
    }

    pub fn recovery_offset(&self) -> Option<u64> {
        if self.has_recovery_offset() {
            Some(self.0.recovery_offset_or_journal_tail().read())
        } else {
            None
        }
    }

    pub fn journal_tail(&self) -> Option<u64> {
        if self.has_journal() {
            Some(self.0.recovery_offset_or_journal_tail().read())
        } else {
            None
        }
    }
}

impl<S: AsRef<[u8]>> Superblock for SuperblockVersion1<S> {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version()
    }

    fn major_version(&self) -> u32 {
        self.0.major_version().read()
    }

    fn array_uuid(&self) -> ArrayUuid {
        ArrayUuid::from_u8_16(self.0.array_uuid())
    }
}