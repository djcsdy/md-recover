use crate::block_device::InMemoryBlockDevice;
use crate::ext::ReadAll;
use crate::ext4::fs::Ext4Fs;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

fn zero_32mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("test_data/zero-32MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn random_2mb_zero_30mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("test_data/random-2MB-zero-30MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn ext4_100mb_empty_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("test_data/ext4-100MB-empty.gz");
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
    Ext4Fs::open(ext4_100mb_empty_device()?)?;
    Ok(())
}
