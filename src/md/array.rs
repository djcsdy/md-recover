use crate::block_device::{BlockCount, BlockDevice, BlockDeviceReader, BlockNumber, BlockSize};
use crate::ext::MultiMap;
use crate::md::definition::MdArrayDefinition;
use crate::md::diagnosis::Diagnosis;
use crate::md::format::MdFormat;
use crate::md::units::SectorNumber;
use crate::md::MdDevice;
use itertools::{Either, EitherOrBoth, Itertools};
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
        let roles = devices
            .iter()
            .map(|device| {
                device
                    .superblock
                    .as_option()
                    .map(|superblock| superblock.device_roles())
            })
            .reduce(|acc, roles| {
                acc.and_then(|acc| {
                    roles.and_then(|roles| {
                        acc.into_iter()
                            .zip_longest(roles)
                            .map(|pair| match pair {
                                EitherOrBoth::Both(left, right) => {
                                    if left.is_spare() || right.is_invalid() || left.is_faulty() {
                                        Some(right)
                                    } else if right.is_spare()
                                        || left.is_invalid()
                                        || right.is_faulty()
                                        || left == right
                                    {
                                        Some(left)
                                    } else {
                                        None
                                    }
                                }
                                EitherOrBoth::Left(role) | EitherOrBoth::Right(role) => Some(role),
                            })
                            .collect()
                    })
                })
            })
            .flatten();
        let format = devices
            .iter()
            .map(|device| MdFormat::from_superblock(device.superblock.as_ref()))
            .reduce(|a, b| if a == b { a } else { None })
            .flatten();
        let new_format = devices
            .iter()
            .map(|device| MdFormat::from_superblock_reshape_status(device.superblock.as_ref()))
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
                            roles
                                .as_ref()
                                .and_then(|roles| roles.get(index))
                                .and_then(|role| role.device_number())
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
                format,
                new_format,
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
        // FIXME: Handle the boundary between the new format and the old.
        Ok(self
            .definition
            .new_format
            .as_ref()
            .or(self.definition.format.as_ref())
            .ok_or(io::ErrorKind::InvalidData)?
            .data_sector_count()
            .ok_or(io::ErrorKind::InvalidData)?
            .as_block_count())
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < 512 {
            Err(io::ErrorKind::InvalidInput)?;
        }

        // FIXME: Handle the boundary between the new format and the old.
        let format = self
            .definition
            .new_format
            .as_ref()
            .or(self.definition.format.as_ref())
            .ok_or(io::ErrorKind::InvalidData)?;

        let block = format.algorithm.read_sector(
            SectorNumber::from_block_number(block_number),
            format.chunk_size,
            format.device_count,
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
