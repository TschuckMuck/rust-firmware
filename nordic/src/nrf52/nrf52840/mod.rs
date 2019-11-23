use core::convert::From;
use core::convert::Into;
use nostd::io;
use nostd::registers::{Read, Register, Write};
use nostd::uart::{BaudRate, Configuration, Configure, Parity, StopBits};

pub enum Port {
    P0,
    P1,
}

#[derive(Clone)]
pub enum Pin {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
    P16,
    P17,
    P18,
    P19,
    P20,
    P21,
    P22,
    P23,
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
    P31,
}

pub struct GpIo {
    port: Port,
    pin: Pin,
}

pub struct Input {
    gpio: GpIo,
}

pub struct Output {
    gpio: GpIo,
}

pub struct Uart {
    rx: Input,
    tx: Output,
    cfg: Option<Configuration>,
}

impl io::Read for Uart {
    fn read_byte(&mut self) -> Result<u8, io::IoError> {
        if !self.is_rx_rdy() {
            Err(io::IoError::NoDataAvailable)
        } else {
            self.clear_rx_rdy();
            Ok(Register::<u32>::new(Self::BASE_ADDRESS + (Self::RXD_OFFSET as usize)).read() as u8)
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::IoError> {
        let mut bytes_read: usize = 0;
        for i in 0..buf.len() {
            match self.read_byte() {
                Ok(byte) => {
                    buf[i] = byte;
                    bytes_read += 1;
                }
                Err(e) => match e {
                    io::IoError::NoDataAvailable => break,
                    _ => return Err(e),
                },
            };
        }
        Ok(bytes_read)
    }
}

impl io::Write for Uart {
    fn write_byte(&mut self, byte: u8) -> Result<(), io::IoError> {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::TXD_OFFSET as usize)).write(byte as u32);
        // wait till byte was transfered
        while (!self.is_tx_rdy()) {}
        // clear tx ready event
        self.clear_tx_rdy();
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, io::IoError> {
        let mut written_bytes: usize = 0;
        for byte in buf.iter() {
            match self.write_byte(*byte) {
                Ok(v) => {
                    written_bytes += 1;
                }
                Err(e) => return Err(e),
            };
        }
        Ok(written_bytes)
    }

    fn flush(&mut self) -> Result<(), io::IoError> {
        // Attention: due to the fact that no internal buffering is used this function is not needed.
        Ok(())
    }
}

impl Uart {
    const BASE_ADDRESS: usize = 0x40002000;
    const ENABLE_OFFSET: u32 = 0x500;
    const ENABLE: u32 = 0x4;
    const CONFIG_OFFSET: u32 = 0x56C;
    const TASKS_START_RX_OFFSET: u32 = 0x0;
    const TASKS_START_TX_OFFSET: u32 = 0x8;
    const BAUD_RATE_OFFSET: u32 = 0x524;
    const PSEL_RXD_OFFSET: u32 = 0x514;
    const PSEL_TXD_OFFSET: u32 = 0x50C;
    const EVENTS_RXRDY: u32 = 0x108;
    const EVENTS_TXRDY: u32 = 0x11C;
    const RXD_OFFSET: u32 = 0x518;
    const TXD_OFFSET: u32 = 0x51C;

    pub fn new(rx: GpIo, tx: GpIo) -> Self {
        let mut cfg_rx: Register<u32> =
            Register::new(Self::BASE_ADDRESS + (Self::PSEL_RXD_OFFSET as usize));
        cfg_rx.write(Self::gpio_to_psel(&rx));

        let mut cfg_tx: Register<u32> =
            Register::new(Self::BASE_ADDRESS + (Self::PSEL_TXD_OFFSET as usize));
        cfg_tx.write(Self::gpio_to_psel(&tx));

        Uart {
            rx: rx.into(),
            tx: tx.into(),
            cfg: None,
        }
    }

    pub fn enable(&self) {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::ENABLE_OFFSET as usize))
            .write(Self::ENABLE);
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::TASKS_START_RX_OFFSET as usize))
            .write(0x01);
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::TASKS_START_TX_OFFSET as usize))
            .write(0x01);
    }

    fn gpio_to_psel(gpio: &GpIo) -> u32 {
        let port: u32 = match gpio.port {
            Port::P0 => 0,
            Port::P1 => 1 << 5,
        };
        let pin: u32 = gpio.pin.clone().into();
        port | pin
    }

    fn baudrate(&mut self, value: BaudRate) {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::BAUD_RATE_OFFSET as usize)).write(
            match value {
                BaudRate::Baud1200 => 0x0004F000u32,
                BaudRate::Baud2400 => 0x0009D000u32,
                BaudRate::Baud4800 => 0x0013B000u32,
                BaudRate::Baud9600 => 0x00275000u32,
                BaudRate::Baud19200 => 0x004EA000u32,
                BaudRate::Baud38400 => 0x009D5000u32,
                BaudRate::Baud57600 => 0x00EBF000u32,
                BaudRate::Baud115200 => 0x01D7E000u32,
                BaudRate::Baud230400 => 0x03AFB000u32,
                BaudRate::Baud460800 => 0x075F7000u32,
                BaudRate::Baud921600 => 0x0EBED000u32,
                BaudRate::Baud1M => 0x10000000u32,
                BaudRate::Other(v) => v as u32,
            },
        )
    }

    fn is_rx_rdy(&self) -> bool {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::EVENTS_RXRDY as usize)).read() != 0
    }

    fn clear_rx_rdy(&mut self) {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::EVENTS_RXRDY as usize)).write(0);
    }

    fn is_tx_rdy(&self) -> bool {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::EVENTS_TXRDY as usize)).read() != 0
    }

    fn clear_tx_rdy(&mut self) {
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::EVENTS_TXRDY as usize)).write(0);
    }
}

impl ::nostd::uart::Configure for Uart {
    fn configure(&mut self, cfg: Configuration) {
        self.baudrate(cfg.baud_rate);
        let hw_flow_control: u32 = match cfg.hw_flow_control {
            true => 1,
            false => 0,
        };
        let parity: u32 = match cfg.parity {
            Parity::Even => 0x07,
            Parity::None => 0x00,
            // Attention: Odd parity is not supported by the nrf52840 HW
            Parity::Odd => {
                assert!(false);
                // TODO: Check if its possible to assert/error without fake return value
                0
            }
        } << 1;
        let stop_bits: u32 = match cfg.stop_bits {
            StopBits::One => 0,
            StopBits::Two => 1,
        } << 4;
        Register::<u32>::new(Self::BASE_ADDRESS + (Self::CONFIG_OFFSET as usize))
            .write(hw_flow_control | parity | stop_bits);
    }
}

impl Port {
    fn base_address(&self) -> u32 {
        match self {
            Port::P0 => 0x50000000,
            Port::P1 => 0x50000300,
        }
    }
}

impl Pin {
    fn bit_pos(&self) -> u32 {
        self.clone().into()
    }
}

impl Into<u32> for Pin {
    fn into(self) -> u32 {
        match self {
            Pin::P0 => 0,
            Pin::P1 => 1,
            Pin::P2 => 2,
            Pin::P3 => 3,
            Pin::P4 => 4,
            Pin::P5 => 5,
            Pin::P6 => 6,
            Pin::P7 => 7,
            Pin::P8 => 8,
            Pin::P9 => 9,
            Pin::P10 => 10,
            Pin::P11 => 11,
            Pin::P12 => 12,
            Pin::P13 => 13,
            Pin::P14 => 14,
            Pin::P15 => 15,
            Pin::P16 => 16,
            Pin::P17 => 17,
            Pin::P18 => 18,
            Pin::P19 => 19,
            Pin::P20 => 20,
            Pin::P21 => 21,
            Pin::P22 => 22,
            Pin::P23 => 23,
            Pin::P24 => 24,
            Pin::P25 => 25,
            Pin::P26 => 26,
            Pin::P27 => 27,
            Pin::P28 => 28,
            Pin::P29 => 29,
            Pin::P30 => 30,
            Pin::P31 => 31,
        }
    }
}

impl GpIo {
    const PULL_UP: u32 = 0xC;
    const ENABLE_INPUT: u32 = 0x0;
    const ENABLE_OUTPUT: u32 = 0x3;
    const IN_OFFSET: u32 = 0x510;
    const DIRCLR_OFFSET: u32 = 0x51C;
    const DIRSET_OFFSET: u32 = 0x518;
    const OUTSET_OFFSET: u32 = 0x508;
    const OUTCLR_OFFSET: u32 = 0x50C;
    const CNF_BASE_OFFSET: u32 = 0x700;

    pub fn new(port: Port, pin: Pin) -> Self {
        GpIo { port, pin }
    }

    fn read(&self) -> bool {
        let r: Register<u32> = Register::new((self.port.base_address() + Self::IN_OFFSET) as usize);
        ((r.read() >> self.pin.bit_pos()) & 0x1) == 0
    }

    fn set(&mut self) {
        Register::<u32>::new((self.port.base_address() + Self::OUTSET_OFFSET) as usize)
            .write(1 << self.pin.bit_pos())
    }

    fn clear(&mut self) {
        Register::<u32>::new((self.port.base_address() + Self::OUTCLR_OFFSET) as usize)
            .write(1 << self.pin.bit_pos())
    }

    fn dir_set(&mut self) {
        Register::<u32>::new((self.port.base_address() + Self::DIRSET_OFFSET) as usize)
            .write(1 << self.pin.bit_pos())
    }

    fn dir_clear(&mut self) {
        Register::<u32>::new((self.port.base_address() + Self::DIRCLR_OFFSET) as usize)
            .write(1 << self.pin.bit_pos())
    }

    fn config(&mut self, value: u32) {
        Register::<u32>::new(
            (self.port.base_address() + Self::CNF_BASE_OFFSET + self.pin.bit_pos() * 4) as usize,
        )
        .write(value)
    }
}

impl From<GpIo> for Input {
    fn from(mut gpio: GpIo) -> Self {
        gpio.dir_clear();
        gpio.config(GpIo::ENABLE_INPUT | GpIo::PULL_UP);
        Input { gpio }
    }
}

impl From<GpIo> for Output {
    fn from(mut gpio: GpIo) -> Self {
        gpio.dir_set();
        gpio.config(GpIo::ENABLE_OUTPUT);
        Output { gpio }
    }
}

impl nostd::gpio::Input for Input {
    fn read(&self) -> bool {
        self.gpio.read()
    }
}

impl nostd::gpio::Output for Output {
    fn on(&mut self) {
        self.gpio.clear();
    }
    fn off(&mut self) {
        self.gpio.set();
    }
    fn toggle(&mut self) {
        match self.gpio.read() {
            true => self.off(),
            false => self.on(),
        };
    }
}
