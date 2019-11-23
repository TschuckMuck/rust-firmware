//! This example enables the 4 leds on the nrf52840dk and the 4 buttons
//! while a button is pressed the associted led will light up
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;

use nordic::nrf52::nrf52840;
use nordic::nrf52::nrf52840::{GpIo, Pin, Port, Uart};
use nostd::gpio::{Input, Output};
use nostd::io::{Read, Write};
use nostd::uart::BaudRate::Baud115200;
use nostd::uart::{BaudRate, Configuration, Configure, Parity, StopBits};

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
        match uart.read_byte() {
            Ok(byte) => {
                uart.write_byte(byte);
            }
            Err(e) => {
                error_count += 1;
            }
        };
    }
}
