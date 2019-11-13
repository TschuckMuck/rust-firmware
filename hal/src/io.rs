use core::result::Result;

pub enum IoError {}

trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
}

trait Write {
    fn write(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;

    fn flush(&mut self) -> Result<(), IoError>;
}
