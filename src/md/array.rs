use crate::block_device::{BlockCount, BlockDevice, BlockDeviceReader, BlockNumber, BlockSize};
use crate::ext::MultiMap;
use crate::md::config::MdConfig;
use crate::md::definition::MdArrayDefinition;
use crate::md::diagnosis::Diagnosis;
use crate::md::units::SectorNumber;
use crate::md::MdDevice;
use itertools::{Either, Itertools};
use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

pub struct MdArray<D>
where
    D: BlockDevice + Read + Seek,
{
    definition: Rc<MdArrayDefinition<D>>,
}

impl<D> MdArray<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(devices: impl IntoIterator<Item = impl Into<Rc<MdDevice<D>>>>) -> Self {
        let devices = devices.into_iter().map(Into::into).collect_vec();
        let config = devices
            .iter()
            .map(|device| MdConfig::from_superblock(device.superblock.as_ref()))
            .reduce(|a, b| if a == b { a } else { None })
            .flatten();
        let (devices, inactive_devices): (HashMap<_, _>, Vec<_>) =
            HashMap::from_multi_iter(devices.into_iter().map(|device| {
                (
                    device
                        .superblock
                        .as_option()
                        .map(|superblock| superblock.device_role_index())
                        .and_then(|index| {
                            config.as_ref().and_then(|config| {
                                config
                                    .device_roles
                                    .get(index)
                                    .and_then(|role| role.device_number())
                            })
                        }),
                    device,
                )
            }))
            .into_iter()
            .partition_map(|(device_number, devices)| {
                match (device_number, devices.len()) {
                    (Some(device_number), 1) => Either::Left((device_number, devices[0].clone())),
                    _ => Either::Right(devices),
                }
            });
        let inactive_devices = inactive_devices.into_iter().flatten().collect_vec();

        Self {
            definition: Rc::new(MdArrayDefinition {
                config,
                devices,
                inactive_devices,
            }),
        }
    }

    pub fn diagnose(&self) -> Diagnosis {
        self.definition.diagnose()
    }
}

impl<D> BlockDevice for MdArray<D>
where
    D: BlockDevice + Read + Seek,
{
    fn block_size(&self) -> io::Result<BlockSize> {
        Ok(BlockSize(512))
    }

    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(self
            .definition
            .config
            .as_ref()
            .ok_or(io::ErrorKind::InvalidData)?
            .data_sector_count()
            .ok_or(io::ErrorKind::InvalidData)?
            .as_block_count())
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < 512 {
            Err(io::ErrorKind::InvalidInput)?;
        }

        let config = self
            .definition
            .config
            .as_ref()
            .ok_or(io::ErrorKind::InvalidData)?;

        let block = config.algorithm.read_sector(
            SectorNumber::from_block_number(block_number),
            config.chunk_size,
            config.device_count,
            |device_number, sector_number, buf| {
                if buf.len() < 512 {
                    Err(io::ErrorKind::InvalidInput)?;
                }

                let device = self
                    .definition
                    .devices
                    .get(&device_number)
                    .ok_or(io::ErrorKind::InvalidInput)?
                    .as_ref()
                    .try_clone()?;
                let mut reader = BlockDeviceReader::new(device);
                reader.seek(SeekFrom::Start(
                    u64::from(sector_number)
                        .checked_mul(512)
                        .ok_or(io::ErrorKind::InvalidInput)?,
                ))?;
                reader.read_exact(&mut buf[..512])?;
                Ok(512)
            },
        )?;
        buf.copy_from_slice(&block);
        Ok(512)
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            definition: self.definition.clone(),
        })
    }
}
