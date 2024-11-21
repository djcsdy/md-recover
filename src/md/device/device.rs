use crate::block_device::{BlockDevice, NativeBlockDevice};
use crate::md::device::id::MdDeviceId;
use crate::md::device::superblock::MdDeviceSuperblock;
use crate::md::superblock::{read_superblock_version_0, SuperblockVersion1};
use std::ffi::OsStr;
use std::io::{Result, SeekFrom};
use std::path::Path;

pub struct MdDevice {
    pub id: MdDeviceId,
    pub superblock: MdDeviceSuperblock,
    device: Box<dyn BlockDevice>,
}

impl MdDevice {
    const MIN_DEVICE_SIZE: u64 = 12288;
    const MIN_SUPERBLOCK_0_DEVICE_SIZE: u64 = 65536;
}

impl MdDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let device = NativeBlockDevice::open_path(path.as_ref())?;
        Self::from_block_device(Box::new(device), Some(path.as_ref().as_os_str()))
    }
}

impl<'md_device> MdDevice {
    pub fn from_block_device<S: AsRef<OsStr>>(
        mut device: Box<dyn BlockDevice>,
        user_reference: Option<S>,
    ) -> Result<Self> {
        let size = device.size()?;

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
            match SuperblockVersion1::read(&mut device, minor_version) {
                Ok(superblock) => {
                    return Ok(Self {
                        id,
                        superblock: MdDeviceSuperblock::Superblock(Box::new(superblock)),
                        device,
                    });
                }
                Err(_) => {}
            }
        }

        if size >= Self::MIN_SUPERBLOCK_0_DEVICE_SIZE {
            device.seek(SeekFrom::Start((size & !65535) - 65536))?;
            match read_superblock_version_0(&mut device) {
                Ok(superblock) => {
                    return Ok(Self {
                        id,
                        superblock: MdDeviceSuperblock::Superblock(Box::new(superblock)),
                        device,
                    })
                }
                Err(_) => {}
            }
        }

        Ok(Self {
            id,
            superblock: MdDeviceSuperblock::Missing,
            device,
        })
    }
}
