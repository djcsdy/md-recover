pub use r#trait::Superblock;
pub use version_0_le::SuperblockVersion0Le;
pub use version_1::SuperblockVersion1;

mod version_0_le;
mod version_1;
mod r#trait;
