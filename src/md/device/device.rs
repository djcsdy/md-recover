use crate::block_device::{BlockDevice, NativeBlockDevice};
use crate::md::device::id::MdDeviceId;
use crate::md::device::superblock::MdDeviceSuperblock;
use crate::md::superblock::{SuperblockVersion0, SuperblockVersion1};
use std::ffi::OsStr;
use std::io;
use std::io::SeekFrom;
use std::path::Path;

pub struct MdDevice<D: BlockDevice> {
    pub id: MdDeviceId,
    pub superblock: MdDeviceSuperblock,
    device: D,
}

impl<D: BlockDevice> MdDevice<D> {
    const MIN_DEVICE_SIZE: u64 = 12288;
    const MIN_SUPERBLOCK_0_DEVICE_SIZE: u64 = 65536;
}

impl MdDevice<NativeBlockDevice> {
    pub fn open_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let device = NativeBlockDevice::open_path(path.as_ref())?;
        Self::from_block_device(device, Some(path.as_ref().as_os_str()))
    }
}

impl<D: BlockDevice> MdDevice<D> {
    pub fn from_block_device<S: AsRef<OsStr>>(
        mut device: D,
        user_reference: Option<S>,
    ) -> io::Result<Self> {
        let size = device.block_count()?.size_bytes(device.block_size()?);

        let id = MdDeviceId::new(user_reference);

        if size < Self::MIN_DEVICE_SIZE {
            return Ok(Self {
                id,
                superblock: MdDeviceSuperblock::TooSmall,
                device,
            });
        }

        for (minor_version, offset) in [(2, 8 << 9), (1, 0), (0, (((size >> 9) - 16) & !7) << 9)] {
            device.seek(SeekFrom::Start(offset))?;
            if let Ok(superblock) = SuperblockVersion1::read(&mut device, minor_version) {
                return Ok(Self {
                    id,
                    superblock: MdDeviceSuperblock::Superblock(Box::new(superblock)),
                    device,
                });
            }
        }

        if size >= Self::MIN_SUPERBLOCK_0_DEVICE_SIZE {
            device.seek(SeekFrom::Start((size & !65535) - 65536))?;
            if let Ok(superblock) = SuperblockVersion0::read(&mut device) {
                return Ok(Self {
                    id,
                    superblock: MdDeviceSuperblock::Superblock(Box::new(superblock)),
                    device,
                });
            }
        }

        Ok(Self {
            id,
            superblock: MdDeviceSuperblock::Missing,
            device,
        })
    }
}
