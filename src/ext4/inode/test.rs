use crate::ext4::inode::flags::Flags;
use crate::ext4::inode::{FileMode, FileType, Inode, Permissions};
use crate::ext4::superblock::Superblock;
use crate::ext4::units::BlockCount;
use chrono::{DateTime, NaiveDate, NaiveTime};

const SUPERBLOCK: &[u8] = include_bytes!("test_data/superblock");
const ROOT: &[u8] = include_bytes!("test_data/root");

#[test]
fn root() {
    let superblock = Superblock::new(SUPERBLOCK);
    let inode = Inode::new(&superblock, 2, ROOT);
    assert_eq!(
        inode.file_mode(),
        FileMode::from_file_type_and_permissions(
            FileType::Directory,
            Permissions::USER_READ
                | Permissions::USER_WRITE
                | Permissions::USER_EXECUTE
                | Permissions::GROUP_READ
                | Permissions::GROUP_EXECUTE
                | Permissions::OTHER_READ
                | Permissions::OTHER_EXECUTE
        )
    );
    assert_eq!(inode.owner_user_id(), 0);
    assert_eq!(inode.file_size_bytes(), 4096);
    assert_eq!(
        inode.access_time(),
        NaiveDate::from_ymd_opt(2025, 11, 11)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
            .and_utc()
    );
    assert_eq!(
        inode.change_time(),
        NaiveDate::from_ymd_opt(2025, 11, 11)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
            .and_utc()
    );
    assert_eq!(
        inode.modified_time(),
        NaiveDate::from_ymd_opt(2025, 11, 11)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
            .and_utc()
    );
    assert_eq!(inode.delete_time(), DateTime::UNIX_EPOCH);
    assert_eq!(inode.group_id(), 0);
    assert_eq!(inode.links_count(), 3);
    assert_eq!(inode.block_count(), BlockCount(8));
    assert_eq!(inode.flags(), Flags::HAS_EXTENTS);
    assert_eq!(inode.version(), 0);
    assert_eq!(
        inode.blocks_buffer().as_slice(),
        &[
            0x0a, 0xf3, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
    assert_eq!(inode.generation(), 0);
    assert_eq!(inode.file_acl(), 0);
    assert_eq!(inode.checksum(), 0xDA6E700E);
    assert_eq!(
        inode.creation_time(),
        NaiveDate::from_ymd_opt(2025, 11, 11)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
            .and_utc()
    );
    assert_eq!(inode.project_id(), 0);
}
