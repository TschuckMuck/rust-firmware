#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;

use hal::gpio::{In, Out};
use hal::io::{Read, Write};
use hal::uart::BaudRate::Baud115200;
use hal::uart::{BaudRate, Configuration, Configure, Parity, StopBits};
use nordic::nrf52840::{GpIo, Input, Output, Pin, Port, Uart};

#[entry]
fn main() -> ! {
    let mut error_count = 0;
    let mut uart = Uart::new(GpIo::new(Port::P0, Pin::P8), GpIo::new(Port::P0, Pin::P6));
    uart.configure(Configuration {
        baud_rate: Baud115200,
        stop_bits: StopBits::One,
        parity: Parity::None,
        hw_flow_control: false,
    });
    uart.enable();
    loop {
        let s = "Hello World\n";
        uart.write(s.as_bytes());
    }
}
