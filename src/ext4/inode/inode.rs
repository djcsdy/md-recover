use crate::ext::WideUnsigned;
use crate::ext4::inode::flags::Flags;
use crate::ext4::inode::time::decode_extra_time;
use crate::ext4::inode::FileMode;
use crate::parser::bytes::take_parse;
use crate::parser::number::{le_u16_or_default_eof, le_u32_or_default_eof};
use chrono::{DateTime, Utc};
use nom::multi::many;
use nom::number::{le_u16, le_u32};
use nom::{IResult, Parser};

const NUM_BLOCKS: usize = 15;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Inode {
    pub file_mode: FileMode,
    pub owner_user_id: u32,
    pub file_size_bytes: u64,
    pub access_time: DateTime<Utc>,
    pub change_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
    pub delete_time: DateTime<Utc>,
    pub group_id: u32,
    pub links_count: u16,
    pub block_count: u64,
    pub flags: Flags,
    pub version: u64,
    pub blocks: [u32; NUM_BLOCKS],
    pub generation: u32,
    pub file_acl: u64,
    pub checksum: u32,
    pub creation_time: DateTime<Utc>,
    pub project_id: u32,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct ExtraFields {
    checksum_high: u16,
    change_time_extra: u32,
    modified_time_extra: u32,
    access_time_extra: u32,
    creation_time_low: u32,
    creation_time_extra: u32,
    version_high: u32,
    project_id: u32,
}

impl Inode {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, file_mode_bits) = le_u16().parse(input)?;
        let (input, user_id_low) = le_u16().parse(input)?;
        let (input, size_low) = le_u32().parse(input)?;
        let (input, access_time_low) = le_u32().parse(input)?;
        let (input, change_time_low) = le_u32().parse(input)?;
        let (input, modified_time_low) = le_u32().parse(input)?;
        let (input, delete_time) = le_u32().parse(input)?;
        let (input, group_id_low) = le_u16().parse(input)?;
        let (input, links_count) = le_u16().parse(input)?;
        let (input, block_count_low) = le_u32().parse(input)?;
        let (input, flags_bits) = le_u32().parse(input)?;
        let (input, version_low) = le_u32().parse(input)?; // Linux-specific
        let (input, blocks): (_, Vec<u32>) = many(NUM_BLOCKS, le_u32()).parse(input)?;
        let (input, generation) = le_u32_or_default_eof(0).parse(input)?;
        let (input, file_acl_low) = le_u32_or_default_eof(0).parse(input)?;
        let (input, size_high) = le_u32_or_default_eof(0).parse(input)?;
        let (input, _obsolete_fragment_address) = le_u32_or_default_eof(0).parse(input)?;

        // Linux-specific:
        let (input, block_count_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, file_acl_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, user_id_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, group_id_high) = le_u16_or_default_eof(0).parse(input)?;
        let (input, checksum_low) = le_u16_or_default_eof(0).parse(input)?;
        let (input, _) = le_u16_or_default_eof(0).parse(input)?;

        let (input, extra_isize) = le_u16_or_default_eof(0).parse(input)?;
        let (rest, extra) = take_parse(extra_isize - 2, |input| -> IResult<_, _> {
            let (input, checksum_high) = le_u16_or_default_eof(0).parse(input)?;
            let (input, change_time_extra) = le_u32_or_default_eof(0).parse(input)?;
            let (input, modified_time_extra) = le_u32_or_default_eof(0).parse(input)?;
            let (input, access_time_extra) = le_u32_or_default_eof(0).parse(input)?;
            let (input, creation_time_low) = le_u32_or_default_eof(0).parse(input)?;
            let (input, creation_time_extra) = le_u32_or_default_eof(0).parse(input)?;
            let (input, version_high) = le_u32_or_default_eof(0).parse(input)?;
            let (rest, project_id) = le_u32_or_default_eof(0).parse(input)?;

            Ok((
                rest,
                ExtraFields {
                    checksum_high,
                    change_time_extra,
                    modified_time_extra,
                    access_time_extra,
                    creation_time_low,
                    creation_time_extra,
                    version_high,
                    project_id,
                },
            ))
        })
        .parse(input)?;

        Ok((
            rest,
            Self {
                file_mode: FileMode::from(file_mode_bits),
                owner_user_id: u32::from_low_high(user_id_low, user_id_high),
                file_size_bytes: u64::from_low_high(size_low, size_high),
                access_time: decode_extra_time(access_time_low, extra.access_time_extra),
                change_time: decode_extra_time(change_time_low, extra.change_time_extra),
                modified_time: decode_extra_time(modified_time_low, extra.modified_time_extra),
                delete_time: decode_extra_time(delete_time, 0),
                group_id: u32::from_low_high(group_id_low, group_id_high),
                links_count,
                block_count: u64::from_low_high(block_count_low, u32::from(block_count_high)),
                flags: Flags::from_bits_retain(flags_bits),
                version: u64::from_low_high(version_low, extra.version_high),
                blocks: blocks.try_into().unwrap(),
                generation,
                file_acl: u64::from_low_high(file_acl_low, u32::from(file_acl_high)),
                checksum: u32::from_low_high(checksum_low, extra.checksum_high),
                creation_time: decode_extra_time(
                    extra.creation_time_low,
                    extra.creation_time_extra,
                ),
                project_id: extra.project_id,
            },
        ))
    }
}
