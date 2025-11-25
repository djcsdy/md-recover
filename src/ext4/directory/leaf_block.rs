use crate::ext4::crc::EXT4_CRC32C;
use crate::ext4::directory::dir_entry_tail;

pub struct Ext4DirectoryLeafBlock<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> Ext4DirectoryLeafBlock<S> {
    pub(super) fn from_block_and_checksum_seed(
        block: S,
        checksum_seed: Option<u32>,
    ) -> Option<Self> {
        if let Some(checksum_seed) = checksum_seed {
            let block = block.as_ref();
            let tail_size = dir_entry_tail::layout::SIZE.unwrap();
            let tail_offset = block.len() - tail_size;

            let tail = dir_entry_tail::layout::View::new(&block[tail_offset..]);
            if tail.reserved_zero_1().read() != 0
                || usize::from(tail.record_length().read()) != tail_size
                || tail.reserved_zero_2().read() != 0
                || tail.reserved_file_type().read() != 0xde
            {
                return None;
            }

            let expected_checksum = {
                let mut digest = EXT4_CRC32C.digest_with_initial(checksum_seed.reverse_bits());
                digest.update(&block[..tail_offset]);
                digest.finalize()
            };

            if tail.checksum().read() != expected_checksum {
                return None;
            }
        }

        Some(Self(block))
    }
}
