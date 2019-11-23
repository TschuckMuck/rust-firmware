pub enum BaudRate {
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
    Baud1M,
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

/// Configuration settings of a `UART` device.
pub struct Configuration {
    pub baud_rate: BaudRate,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub hw_flow_control: bool,
}

/// A trait for object which can be configured as `UART`.
pub trait Configure {
    fn configure(&mut self, cfg: Configuration);
}
