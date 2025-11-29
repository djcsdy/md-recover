use derive_more::{AsMut, AsRef, From, Into};
use std::fs::File;
use std::io;

#[derive(Debug, From, Into, AsRef, AsMut)]
pub struct InternalFile(pub File);

impl InternalFile {
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self(self.0.try_clone()?))
    }
}
