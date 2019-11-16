//! This example enables the 4 leds on the nrf52840dk and the 4 buttons
//! while a button is pressed the associted led will light up
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::ptr;

use cortex_m::asm;
use cortex_m_rt::entry;

use hal::gpio::{In, Out};
use nordic::nrf52840dk::{GpIo, Input, Output, Port};

#[entry]
fn main() -> ! {
    let buttons: [Input; 4] = [
        GpIo::new(Port::P0, GpIo::BUTTON1_PIN).into(),
        GpIo::new(Port::P0, GpIo::BUTTON2_PIN).into(),
        GpIo::new(Port::P0, GpIo::BUTTON3_PIN).into(),
        GpIo::new(Port::P0, GpIo::BUTTON4_PIN).into(),
    ];

    let mut leds: [Output; 4] = [
        GpIo::new(Port::P0, GpIo::LED1_PIN).into(),
        GpIo::new(Port::P0, GpIo::LED2_PIN).into(),
        GpIo::new(Port::P0, GpIo::LED3_PIN).into(),
        GpIo::new(Port::P0, GpIo::LED4_PIN).into(),
    ];

    for mut led in leds.iter_mut() {
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
