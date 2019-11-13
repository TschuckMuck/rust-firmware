pub enum BaudRate {
    Baud110,
    Baud150,
    Baud300,
    Baud1200,
    Baud2400,
    Baud4800,
    Baud9600,
    Baud19200,
    Baud38400,
    Baud57600,
    Baud115200,
    Baud230400,
    Baud460800,
    Baud921600,
    Other(usize),
}

pub enum StopBits {
    One,
    Two,
}

pub enum Parity {
    None,
    Even,
    Odd,
}

pub struct Configuration {
    pub baud_rate: Baudrate,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub hw_flow_control: bool,
}

pub trait Uart: ::io::Write + ::io::Read {
    fn configure(&self, cfg: Configuration);
}
