use core::convert::From;
use hal;
use hal::registers::{Read, Register, Write};
//use hal::uart::Uart;

pub enum Port {
    P0,
    P1,
}

pub enum Pin {
    P0,
}

pub struct GpIo {
    pub set: Register<u32>,
    pub clear: Register<u32>,
    pub dir_set: Register<u32>,
    pub dir_clear: Register<u32>,
    pub input: Register<u32>,
    pub config: Register<u32>,
    pub pin: u32,
}

pub struct Input {
    gpio: GpIo,
}

pub struct Output {
    gpio: GpIo,
}

impl GpIo {
    const PULL_UP: u32 = 0xC;
    const PULL_DOWN: u32 = 0x4;
    const ENABLE_INPUT: u32 = 0x0;
    const ENABLE_OUTPUT: u32 = 0x3;

    pub const LED1_PIN: u32 = 13;
    pub const LED2_PIN: u32 = 14;
    pub const LED3_PIN: u32 = 15;
    pub const LED4_PIN: u32 = 16;
    pub const BUTTON1_PIN: u32 = 11;
    pub const BUTTON2_PIN: u32 = 12;
    pub const BUTTON3_PIN: u32 = 24;
    pub const BUTTON4_PIN: u32 = 25;

    pub fn new(port: Port, pin: u32) -> Self {
        let base_address = match port {
            Port::P0 => 0x50000000,
            Port::P1 => 0x50000300,
        };
        GpIo {
            set: Register::new(base_address + 0x508),
            clear: Register::new(base_address + 0x50C),
            dir_set: Register::new(base_address + 0x518),
            dir_clear: Register::new(base_address + 0x51C),
            input: Register::new(base_address + 0x510),
            config: Register::new(base_address + 0x700 + (pin as usize) * 4),
            pin,
        }
    }
}

impl From<GpIo> for Input {
    fn from(mut gpio: GpIo) -> Self {
        gpio.dir_clear.write(1 << gpio.pin);
        let value = gpio.dir_clear.read();
        gpio.config.write(GpIo::ENABLE_INPUT | GpIo::PULL_UP);
        Input { gpio }
    }
}

impl From<GpIo> for Output {
    fn from(mut gpio: GpIo) -> Self {
        gpio.dir_set.write(1 << gpio.pin);
        gpio.config.write(GpIo::ENABLE_OUTPUT);
        Output { gpio }
    }
}

impl hal::gpio::In for Input {
    fn read(&self) -> bool {
        ((self.gpio.input.read() >> self.gpio.pin) & 0x1) == 0
    }
}

impl hal::gpio::Out for Output {
    fn on(&mut self) {
        self.gpio.clear.write(1 << self.gpio.pin);
    }
    fn off(&mut self) {
        self.gpio.set.write(1 << self.gpio.pin);
    }
    fn toggle(&mut self) {
        let is_set = (self.gpio.input.read() & (1u32 >> self.gpio.pin)) > 0;
        match is_set {
            true => self.off(),
            false => self.on(),
        };
    }
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
