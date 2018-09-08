use std::io::{self, Seek, SeekFrom};

pub trait SeekExt: Seek {
    fn whereami(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

impl<S: Seek + ?Sized> SeekExt for S {}
