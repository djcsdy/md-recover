use crate::block_device::BlockDevice;
use crate::ext::LongMul;
use crate::ext4::block_group::BlockGroupDescriptor;
use crate::ext4::inode::Inode;
use crate::ext4::superblock::{CreatorOs, Superblock};
use bitflags::Flags;
use std::io::{Error, ErrorKind, Result, SeekFrom};

pub struct Ext4Fs<D: BlockDevice> {
    device: D,
    pub(super) superblock: Superblock<Vec<u8>>,
    pub(super) group_descriptors: Vec<BlockGroupDescriptor>,
}

impl<D: BlockDevice> Ext4Fs<D> {
    pub fn open(mut device: D) -> Result<Self> {
        device.seek(SeekFrom::Start(1024))?;
        let superblock = Superblock::read(&mut device)?;

        let block_count = superblock.blocks_count();
        let blocks_per_group = u64::from(superblock.blocks_per_group());
        let group_count = usize::try_from(block_count.div_ceil(blocks_per_group))
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

        let group_descriptors_block_index = u64::from(superblock.first_data_block()) + 1;
        device.seek(SeekFrom::Start(
            group_descriptors_block_index * superblock.block_size_bytes(),
        ))?;
        let mut group_descriptors = Vec::with_capacity(group_count);
        for _ in 0..group_count {
            let mut buf = vec![0u8; group_size];
            device.read_exact(&mut buf)?;
            group_descriptors.push(BlockGroupDescriptor::new(buf))
        }

        Ok(Self {
            device,
            superblock,
            group_descriptors,
        })
    }

    pub fn read_root_inode(&mut self) -> Result<Inode> {
        self.read_inode(2)
    }

    fn read_inode(&mut self, inode_number: u32) -> Result<Inode> {
        if inode_number == 0 || inode_number > self.superblock.inodes_count() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        let group_index = (inode_number - 1) / self.superblock.inodes_per_group();
        let index_in_group = (inode_number - 1) % self.superblock.inodes_per_group();

        let group = self
            .group_descriptors
            .get(usize::try_from(group_index).map_err(|_| Error::from(ErrorKind::InvalidInput))?)
            .ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;

        let inode_offset_within_table =
            index_in_group.long_mul(u32::from(self.superblock.inode_size()));
        let inode_byte_offset = group
            .inode_table_block()
            .checked_mul(self.superblock.block_size_bytes())
            .and_then(|base| base.checked_add(inode_offset_within_table))
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

        self.device.seek(SeekFrom::Start(inode_byte_offset))?;
        let mut buffer = vec![0; usize::from(self.superblock.inode_size())];
        self.device.read_exact(&mut buffer)?;
        Ok(Inode::new(buffer))
    }
}
