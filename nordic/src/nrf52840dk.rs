use core::convert::From;
use hal::registers::{Read, Register, Write};

pub enum Port {
    P0,
    P1,
}

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

pub struct UartRegisterBlock {
    pub rx_config: Register<u32>,
    pub tx_config: Register<u32>,
    pub baudrate: Register<u32>,
    pub pin_select_rx: Register<u32>,
    pub pin_select_tx: Register<u32>,
    pub enable: Register<u32>,
    pub rx: Register<u32>,
    pub tx: Register<u32>,
    pub start_rx: Register<u32>,
    pub start_tx: Register<u32>,
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
        let mut r: Register<u32> =
            Register::new((self.port.base_address() + Self::OUTSET_OFFSET) as usize);
        r.write(1 << self.pin.bit_pos())
    }

    fn clear(&mut self) {
        let mut r: Register<u32> =
            Register::new((self.port.base_address() + Self::OUTCLR_OFFSET) as usize);
        r.write(1 << self.pin.bit_pos())
    }

    fn dir_set(&mut self) {
        let mut r: Register<u32> =
            Register::new((self.port.base_address() + Self::DIRSET_OFFSET) as usize);
        r.write(1 << self.pin.bit_pos())
    }

    fn dir_clear(&mut self) {
        let mut r: Register<u32> =
            Register::new((self.port.base_address() + Self::DIRCLR_OFFSET) as usize);
        r.write(1 << self.pin.bit_pos())
    }

    fn config(&mut self, value: u32) {
        let mut r: Register<u32> = Register::new(
            (self.port.base_address() + Self::CNF_BASE_OFFSET + self.pin.bit_pos() * 4) as usize,
        );
        r.write(value)
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

impl hal::gpio::In for Input {
    fn read(&self) -> bool {
        self.gpio.read()
    }
}

impl hal::gpio::Out for Output {
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
