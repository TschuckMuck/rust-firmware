use core::result::Result;

pub enum IoError {
    NoDataAvailable,
}

pub trait Read {
    fn read_byte(&mut self) -> Result<u8, IoError>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
}

pub trait Write {
    fn write_byte(&mut self, byte: u8) -> Result<(), IoError>;

    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;

    fn flush(&mut self) -> Result<(), IoError>;
}
