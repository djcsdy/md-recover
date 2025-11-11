use crate::block_device::BlockDevice;
use crate::ext4::block_group::BlockGroupDescriptor;
use crate::ext4::superblock::{CreatorOs, IncompatibleFeatures, Superblock};
use bitflags::Flags;
use nom::bytes::take;
use nom::multi::many;
use nom::Parser;
use std::io::{Error, ErrorKind, Result, SeekFrom};

pub struct Ext4Fs<D: BlockDevice> {
    device: D,
    superblock: Superblock<Vec<u8>>,
    group_descriptors: Vec<BlockGroupDescriptor>,
}

impl<D: BlockDevice> Ext4Fs<D> {
    pub fn open(mut device: D) -> Result<Self> {
        device.seek(SeekFrom::Start(1024))?;
        let superblock = Superblock::read(&mut device)?;

        let block_count = superblock.blocks_count();
        let blocks_per_group = u64::from(superblock.blocks_per_group());
        let group_count = usize::try_from(block_count / blocks_per_group)
            .map_err(|_| Error::from(ErrorKind::Unsupported))?;
        let group_size = usize::from(superblock.group_descriptor_size());

        if blocks_per_group == 0
            || group_size == 0
            || superblock.inodes_per_group() == 0
            || superblock.creator_os() != CreatorOs::Linux
            || superblock.incompatible_features().contains_unknown_bits()
        {
            return Err(Error::from(ErrorKind::Unsupported));
        }

        let mut group_descriptors = Vec::with_capacity(group_count);
        for _ in 0..group_count {
            let mut buf = vec![0u8; group_size];
            device.read_exact(&mut buf)?;
            let (_, group_descriptor) = BlockGroupDescriptor::parse
                .parse_complete(&buf)
                .map_err(|_| Error::from(ErrorKind::InvalidData))?;
            group_descriptors.push(group_descriptor);
        }

        Ok(Self {
            device,
            superblock,
            group_descriptors,
        })
    }
}
