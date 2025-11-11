use crate::ext::WideUnsigned;
use crate::ext4::block_group::Flags;
use crate::parser::number::{le_u16_or_default, le_u32_or_default};
use nom::{IResult, Needed, Parser};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct BlockGroupDescriptor {
    block_bitmap_block: u64,
    inode_bitmap_block: u64,
    inode_table_block: u64,
    free_block_count: u32,
    free_inode_count: u32,
    used_directories_count: u32,
    flags: Flags,
    exclude_bitmap_block: u64,
    block_bitmap_checksum: u32,
    inode_bitmap_checksum: u32,
    unused_inode_count: u32,
    checksum: u16,
}

impl BlockGroupDescriptor {
    pub fn parse_complete(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, block_bitmap_block_low) = le_u32_or_default(0).parse_complete(input)?;
        let (input, inode_bitmap_block_low) = le_u32_or_default(0).parse_complete(input)?;
        let (input, inode_table_block_low) = le_u32_or_default(0).parse_complete(input)?;
        let (input, free_block_count_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, free_inode_count_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, used_directories_count_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, flags_bits) = le_u16_or_default(0).parse_complete(input)?;
        let (input, exclude_bitmap_block_low) = le_u32_or_default(0).parse_complete(input)?;
        let (input, block_bitmap_checksum_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, inode_bitmap_checksum_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, unused_inode_count_low) = le_u16_or_default(0).parse_complete(input)?;
        let (input, checksum) = le_u16_or_default(0).parse_complete(input)?;
        let (input, block_bitmap_block_high) = le_u32_or_default(0).parse_complete(input)?;
        let (input, inode_bitmap_block_high) = le_u32_or_default(0).parse_complete(input)?;
        let (input, inode_table_block_high) = le_u32_or_default(0).parse_complete(input)?;
        let (input, free_block_count_high) = le_u16_or_default(0).parse_complete(input)?;
        let (input, free_inode_count_high) = le_u16_or_default(0).parse_complete(input)?;
        let (input, used_directories_count_high) = le_u16_or_default(0).parse_complete(input)?;
        let (input, unused_inode_count_high) = le_u16_or_default(0).parse_complete(input)?;
        let (input, exclude_bitmap_block_high) = le_u32_or_default(0).parse_complete(input)?;
        let (input, block_bitmap_checksum_high) = le_u16_or_default(0).parse_complete(input)?;
        let (input, inode_bitmap_checksum_high) = le_u16_or_default(0).parse_complete(input)?;

        Ok((
            input,
            Self {
                block_bitmap_block: u64::from_low_high(
                    block_bitmap_block_low,
                    block_bitmap_block_high,
                ),
                inode_bitmap_block: u64::from_low_high(
                    inode_bitmap_block_low,
                    inode_bitmap_block_high,
                ),
                inode_table_block: u64::from_low_high(
                    inode_table_block_low,
                    inode_table_block_high,
                ),
                free_block_count: u32::from_low_high(free_block_count_low, free_block_count_high),
                free_inode_count: u32::from_low_high(free_inode_count_low, free_inode_count_high),
                used_directories_count: u32::from_low_high(
                    used_directories_count_low,
                    used_directories_count_high,
                ),
                flags: Flags::from_bits_retain(flags_bits),
                exclude_bitmap_block: u64::from_low_high(
                    exclude_bitmap_block_low,
                    exclude_bitmap_block_high,
                ),
                block_bitmap_checksum: u32::from_low_high(
                    block_bitmap_checksum_low,
                    block_bitmap_checksum_high,
                ),
                inode_bitmap_checksum: u32::from_low_high(
                    inode_bitmap_checksum_low,
                    inode_bitmap_checksum_high,
                ),
                unused_inode_count: u32::from_low_high(
                    unused_inode_count_low,
                    unused_inode_count_high,
                ),
                checksum,
            },
        ))
    }
}
