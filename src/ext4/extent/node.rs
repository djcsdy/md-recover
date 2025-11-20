use crate::ext4::extent::extent::Extent;
use crate::ext4::extent::index::ExtentIndex;

pub enum ExtentNode<S: AsRef<[u8]>> {
    Index(ExtentIndex<S>),
    Leaf(Extent<S>),
}
