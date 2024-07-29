use crate::block_device::{BlockDevice, NativeBlockDevice};
use crate::md::device::id::MdDeviceId;
use crate::md::superblock::Superblock;
use crate::md::superblock::{SuperblockVersion0, SuperblockVersion1};
use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result, SeekFrom};
use std::path::Path;

pub struct MdDevice<S: Superblock, D: BlockDevice> {
    pub id: MdDeviceId,
    pub superblock: S,
    pub minor_version: u32,
    device: D,
}

impl<S: Superblock, D: BlockDevice> MdDevice<S, D> {
    const MIN_DEVICE_SIZE: u64 = 12288;
    const MIN_SUPERBLOCK_0_DEVICE_SIZE: u64 = 65536;
}

impl MdDevice<Box<dyn Superblock>, NativeBlockDevice> {
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let device = NativeBlockDevice::open_path(path.as_ref())?;
        Self::from_block_device(device, Some(path.as_ref().as_os_str()))
    }
}

impl<D: BlockDevice> MdDevice<Box<dyn Superblock>, D> {
    pub fn from_block_device<S: AsRef<OsStr>>(
        mut device: D,
        user_reference: Option<S>,
    ) -> Result<Self> {
        let size = device.size()?;

        if size < Self::MIN_DEVICE_SIZE {
            return Err(Error::from(ErrorKind::Other));
        }

        let id = MdDeviceId::new(user_reference);

        for (minor_version, offset) in [(2, 8 << 9), (1, 0), (0, (((size >> 9) - 16) & !7) << 9)] {
            device.seek(SeekFrom::Start(offset))?;
            match SuperblockVersion1::read(&mut device) {
                Ok(superblock) => {
                    return Ok(Self {
                        id,
                        superblock: Box::new(superblock),
                        minor_version,
                        device,
                    });
                }
                Err(_) => {}
            }
        }

        if size >= Self::MIN_SUPERBLOCK_0_DEVICE_SIZE {
            device.seek(SeekFrom::Start((size & !65535) - 65536))?;
            let superblock = SuperblockVersion0::read(&mut device)?;
            let minor_version = superblock.minor_version();

            Ok(Self {
                id,
                superblock: Box::new(superblock),
                minor_version,
                device,
            })
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}