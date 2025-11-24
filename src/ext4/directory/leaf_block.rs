pub struct Ext4DirectoryLeafBlock<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> Ext4DirectoryLeafBlock<S> {
    pub(super) fn new(storage: S) -> Self {
        Self(storage)
    }
}
