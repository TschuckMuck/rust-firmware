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

use hal::gpio::{In, Out};
use nordic::nrf52840::{GpIo, Input, Output, Pin, Port};

#[entry]
fn main() -> ! {
    let buttons: [Input; 4] = [
        GpIo::new(Port::P0, Pin::P11).into(),
        GpIo::new(Port::P0, Pin::P12).into(),
        GpIo::new(Port::P0, Pin::P24).into(),
        GpIo::new(Port::P0, Pin::P25).into(),
    ];

    let mut leds: [Output; 4] = [
        GpIo::new(Port::P0, Pin::P13).into(),
        GpIo::new(Port::P0, Pin::P14).into(),
        GpIo::new(Port::P0, Pin::P15).into(),
        GpIo::new(Port::P0, Pin::P16).into(),
    ];

    for led in leds.iter_mut() {
        led.off();
    }

    loop {
        for index in 0..4 {
            if buttons[index].read() {
                leds[index].on();
            } else {
                leds[index].off();
            }
        }
    }
}
