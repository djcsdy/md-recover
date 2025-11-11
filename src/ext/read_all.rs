use std::io;
use std::io::Read;

pub trait ReadAll {
    fn read_all(&mut self) -> io::Result<Vec<u8>>;
}

impl<R: Read> ReadAll for R {
    fn read_all(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
