use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::superblock::version_1::device_flags::DeviceFlags;
use crate::md::superblock::version_1::features::Features;
use crate::md::superblock::version_1::ppl_info::PplInfo;
use crate::md::superblock::version_1::reshape_status::NestedReshapeStatusVersion1;
use crate::md::superblock::{ArrayUuid, MdDeviceRole, Superblock};
use crate::md::units::{DeviceCount, MetadataEventCount, SectorCount};
use binary_layout::binary_layout;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::cmp::min;
use std::ffi::OsStr;
use std::io;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;

binary_layout!(layout, LittleEndian, {
    magic: u32,
    major_version: u32,
    features: Features as u32,
    pad_0: u32,
    array_uuid: [u8; 16],
    array_name: [u8; 32],
    ctime: u64,
    level: u32,
    layout: u32,
    sectors_per_device: SectorCount<u64> as u64,
    chunk_size: SectorCount<u32> as u32,
    raid_device_count: DeviceCount as u32,
    bitmap_offset_or_ppl_info: [u8; 4],
    reshape_status: NestedReshapeStatusVersion1,
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
    event_count: MetadataEventCount as u64,
    resync_offset: u64,
    superblock_checksum: u32,
    max_devices: DeviceCount as u32,
    pad_3: [u8; 32],
    dev_roles: [u8]
});

pub struct SuperblockVersion1<S: AsRef<[u8]>> {
    buffer: layout::View<S>,
    minor_version: u32,
}

impl SuperblockVersion1<Vec<u8>> {
    pub const MAX_SIZE: usize = 4096;

    pub fn read<R: Read>(mut reader: R, minor_version: u32) -> io::Result<Self> {
        let mut buf = vec![0u8; Self::MAX_SIZE];
        reader.read_exact(&mut buf)?;
        let superblock = Self::new(buf, minor_version);
        if superblock.valid() {
            Ok(superblock)
        } else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }
}

impl<S: AsRef<[u8]>> SuperblockVersion1<S> {
    pub fn new(storage: S, minor_version: u32) -> Self {
        Self {
            buffer: layout::View::new(storage),
            minor_version,
        }
    }

    pub fn valid_magic(&self) -> bool {
        self.buffer.magic().read() == 0xa92b4efc
    }

    pub fn valid_major_version(&self) -> bool {
        self.major_version() == 1
    }

    fn features(&self) -> Features {
        self.buffer.features().read()
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

    pub fn bitmap_offset(&self) -> Option<u32> {
        if self.has_bitmap_offset() {
            Some(LittleEndian::read_u32(
                self.buffer.bitmap_offset_or_ppl_info(),
            ))
        } else {
            None
        }
    }

    pub fn ppl_info(&self) -> Option<PplInfo<&[u8]>> {
        if self.has_ppl() {
            Some(PplInfo::new(self.buffer.bitmap_offset_or_ppl_info()))
        } else {
            None
        }
    }

    pub fn recovery_offset(&self) -> Option<u64> {
        if self.has_recovery_offset() {
            Some(self.buffer.recovery_offset_or_journal_tail().read())
        } else {
            None
        }
    }

    pub fn journal_tail(&self) -> Option<u64> {
        if self.has_journal() {
            Some(self.buffer.recovery_offset_or_journal_tail().read())
        } else {
            None
        }
    }

    fn level(&self) -> u32 {
        self.buffer.level().read()
    }

    fn layout(&self) -> u32 {
        self.buffer.layout().read()
    }
}

impl<S: AsRef<[u8]>> Superblock for SuperblockVersion1<S> {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version()
    }

    fn major_version(&self) -> u32 {
        self.buffer.major_version().read()
    }

    fn minor_version(&self) -> u32 {
        self.minor_version
    }

    fn array_uuid(&self) -> ArrayUuid {
        ArrayUuid::from_u8_16(self.buffer.array_uuid())
    }

    fn array_name(&self) -> Option<&OsStr> {
        Some(OsStr::from_bytes(self.buffer.array_name()))
    }

    fn algorithm(&self) -> MdAlgorithm {
        MdAlgorithm::from_level_and_layout(self.level(), self.layout())
    }

    fn sectors_per_device(&self) -> SectorCount<u64> {
        self.buffer.sectors_per_device().read()
    }

    fn chunk_size(&self) -> SectorCount<u32> {
        self.buffer.chunk_size().read()
    }

    fn raid_device_count(&self) -> DeviceCount {
        self.buffer.raid_device_count().read()
    }

    fn reshape_status(&self) -> Option<ReshapeStatus> {
        if self.features().contains(Features::RESHAPE_ACTIVE) {
            Some(self.buffer.reshape_status().into())
        } else {
            None
        }
    }

    fn event_count(&self) -> MetadataEventCount {
        self.buffer.event_count().read()
    }

    fn device_roles(&self) -> Vec<MdDeviceRole> {
        let count = min(
            self.buffer.max_devices().read().into(),
            self.buffer.dev_roles().len() / size_of::<u16>(),
        );
        let mut buffer = vec![0u16; count];
        self.buffer
            .dev_roles()
            .read_u16_into::<LittleEndian>(&mut buffer)
            .unwrap();
        buffer.into_iter().map(MdDeviceRole::from_u16).collect()
    }
}
