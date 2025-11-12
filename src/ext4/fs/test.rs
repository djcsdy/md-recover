use crate::block_device::InMemoryBlockDevice;
use crate::ext::ReadAll;
use crate::ext4::block_group::{BlockGroupDescriptor, Flags};
use crate::ext4::fs::Ext4Fs;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

fn zero_32mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/zero-32MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn random_2mb_zero_30mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/random-2MB-zero-30MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn ext4_100mb_empty_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/ext4-100MB-empty.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

#[test]
fn open_zero_32mb() -> anyhow::Result<()> {
    assert_eq!(
        Ext4Fs::open(zero_32mb_device()?)
            .err()
            .map(|error| error.kind()),
        Some(io::ErrorKind::InvalidData)
    );
    Ok(())
}

#[test]
fn open_random_2mb_zero_30mb() -> anyhow::Result<()> {
    assert_eq!(
        Ext4Fs::open(random_2mb_zero_30mb_device()?)
            .err()
            .map(|error| error.kind()),
        Some(io::ErrorKind::InvalidData)
    );
    Ok(())
}

#[test]
fn open_100mb_empty() -> anyhow::Result<()> {
    let fs = Ext4Fs::open(ext4_100mb_empty_device()?)?;
    assert_eq!(
        fs.group_descriptors,
        vec![BlockGroupDescriptor {
            block_bitmap_block: 0xe,
            inode_bitmap_block: 0x1e,
            inode_table_block: 0x2e,
            free_block_count: 22954,
            free_inode_count: 25589,
            used_directories_count: 2,
            flags: Flags::INODE_TABLE_ZEROED,
            exclude_bitmap_block: 0,
            block_bitmap_checksum: 0x30609723,
            inode_bitmap_checksum: 0x957502e9,
            unused_inode_count: 0x63f5,
            checksum: 0x67e2,
        }]
    );
    Ok(())
}
