#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::time;
use cortex_m_rt::entry;
use nordic::nrf52::nrf52840;
use nordic::nrf52::nrf52840::{GpIo, Pin, Port, Timer, TimerPeripheral};
use nostd::gpio::{Input, Output};

#[entry]
fn main() -> ! {
    let mut leds: [nrf52840::Output; 4] = [
        GpIo::new(Port::P0, Pin::P13).into(),
        GpIo::new(Port::P0, Pin::P14).into(),
        GpIo::new(Port::P0, Pin::P15).into(),
        GpIo::new(Port::P0, Pin::P16).into(),
    ];

    let one_second = core::time::Duration::from_secs(1);
    let mut timer = Timer::new(TimerPeripheral::Timer0);
    for led in leds.iter_mut() {
        led.off();
    }

    loop {
        for led in leds.iter_mut() {
            led.on();
        }
        timer.sleep(one_second);
        for led in leds.iter_mut() {
            led.off();
        }
        timer.sleep(one_second);
    }
}
