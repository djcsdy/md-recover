use crate::ext::WideUnsigned;
use crate::ext4::block_group::Flags;
use crate::parser::number::{le_u16_or_default_eof, le_u32_or_default_eof};
use nom::bytes::take;
use nom::{IResult, Needed, Parser};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct BlockGroupDescriptor {
    pub block_bitmap_block: u64,
    pub inode_bitmap_block: u64,
    pub inode_table_block: u64,
    pub free_block_count: u32,
    pub free_inode_count: u32,
    pub used_directories_count: u32,
    pub flags: Flags,
    pub exclude_bitmap_block: u64,
    pub block_bitmap_checksum: u32,
    pub inode_bitmap_checksum: u32,
    pub unused_inode_count: u32,
    pub checksum: u16,
}

impl BlockGroupDescriptor {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, block_bitmap_block_low) = le_u32_or_default_eof(0).parse(input)?;
        let (input, inode_bitmap_block_low) = le_u32_or_default_eof(0).parse(input)?;
        let (input, inode_table_block_low) = le_u32_or_default_eof(0).parse(input)?;
        let (input, free_block_count_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, free_inode_count_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, used_directories_count_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, flags_bits) = le_u16_or_default_eof(0).parse(input)?;
        let (input, exclude_bitmap_block_low) = le_u32_or_default_eof(0).parse(input)?;
        let (input, block_bitmap_checksum_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, inode_bitmap_checksum_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, unused_inode_count_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, checksum) = le_u16_or_default_eof(0).parse(input)?;
        let (input, block_bitmap_block_high) = le_u32_or_default_eof(0).parse(input)?;
        let (input, inode_bitmap_block_high) = le_u32_or_default_eof(0).parse(input)?;
        let (input, inode_table_block_high) = le_u32_or_default_eof(0).parse(input)?;
        let (input, free_block_count_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, free_inode_count_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, used_directories_count_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, unused_inode_count_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, exclude_bitmap_block_high) = le_u32_or_default_eof(0).parse(input)?;
        let (input, block_bitmap_checksum_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, inode_bitmap_checksum_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, _) = take(4usize).parse(input)?;

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
