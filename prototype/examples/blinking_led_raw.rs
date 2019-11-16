//! Blinking led example using rust unsafe code and polling a timer.
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use core::ptr;

// GPIO pin-specific constants
const P0_BASE_ADDR: u32 = 0x50000000;
const CONFIG_OFFSET: u32 = 0x700;

const UART_TX_PIN: u32 = 6;
const UART_TX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_TX_PIN * 4);

const UART_RX_PIN: u32 = 8;
const UART_RX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_RX_PIN * 4);

const ENABLE_INPUT: u32 = 0x0;
const ENABLE_OUTPUT: u32 = 0x3;

const LED1_CONFIG: u32 = P0_BASE_ADDR + 0x734;
const OUT_REGISTER: u32 = P0_BASE_ADDR + 0x504;

const TIMER_BASE_ADDR: u32 = 0x40008000;

const TIMER_BIT_MODE: u32 = TIMER_BASE_ADDR + 0x508;
const TIMER_PRESCALER_REGISTER: u32 = TIMER_BASE_ADDR + 0x510;
const TIMER_MODE_REGISTER: u32 = TIMER_BASE_ADDR + 0x504;

const START_ZE_TIMER: u32 = TIMER_BASE_ADDR + 0x000;
const STOP_ZE_TIMER: u32 = TIMER_BASE_ADDR + 0x004;

const TIMER_TRIGGERED: u32 = TIMER_BASE_ADDR + 0x140;
const TIMER_TRIGGER: u32 = TIMER_BASE_ADDR + 0x040;
const TIMER_TIMEOUT: u32 = TIMER_BASE_ADDR + 0x540;
const TIMER_INTERRUPT: u32 = TIMER_BASE_ADDR + 0x304;
const TIMER_DISABLE_INTERRUPT: u32 = TIMER_BASE_ADDR + 0x308;
const TIMER_CONFIG_TRIGGER: u32 = TIMER_BASE_ADDR + 0x200;
const CLEAR_TIMER: u32 = TIMER_BASE_ADDR + 0x00C;

const TIMER_32BIT_BITMODE: u32 = 0x03;
const TIMER_TIMER_MODE: u32 = 0x00;
const TIMER_COUNTER_MODE: u32 = 0x01;
const TIMER_PRESCALE_VALUE: u32 = 0x00;
const CLEAR_TIMER_AFTER_TRIGGER: u32 = 0x01;

const ENABLE_TIMER: u32 = 0x01;
const DISABLE_TIMER: u32 = 0x01;

const CLOCK_BASE_ADDR: u32 = 0x40000000;
const CLOCK_START_OSCILLATOR: u32 = CLOCK_BASE_ADDR + 0x000;
const ENABLE_CLOCK: u32 = 0x01;

#[inline(never)]
fn delay(ms: u32) {
    unsafe {
        // init clock
        ptr::write_volatile(CLOCK_START_OSCILLATOR as *mut u32, ENABLE_CLOCK);

        // init timer
        ptr::write_volatile(TIMER_BIT_MODE as *mut u32, TIMER_32BIT_BITMODE);
        ptr::write_volatile(TIMER_MODE_REGISTER as *mut u32, TIMER_TIMER_MODE);
        ptr::write_volatile(TIMER_PRESCALER_REGISTER as *mut u32, TIMER_PRESCALE_VALUE);

        // enable interrupts
        cortex_m::interrupt::enable();
        ptr::write_volatile(TIMER_INTERRUPT as *mut u32, (0x3F << 16));

        ptr::write_volatile(TIMER_TIMEOUT as *mut u32, ms);
        ptr::write_volatile(TIMER_CONFIG_TRIGGER as *mut u32, CLEAR_TIMER_AFTER_TRIGGER);

        ptr::write_volatile(START_ZE_TIMER as *mut u32, ENABLE_TIMER);

        while ptr::read_volatile(TIMER_TRIGGERED as *mut u32) != 1 {
            asm::nop();
        }

        ptr::write_volatile(STOP_ZE_TIMER as *mut u32, DISABLE_TIMER);
        ptr::write_volatile(CLEAR_TIMER as *mut u32, 1);
        ptr::write_volatile(TIMER_TRIGGERED as *mut u32, 0);
        ptr::write_volatile(TIMER_DISABLE_INTERRUPT as *mut u32, 0);
    }
}

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    unsafe {
        // init led output register
        ptr::write_volatile(LED1_CONFIG as *mut u32, ENABLE_OUTPUT);
    }
    loop {
        unsafe {
            ptr::write_volatile(OUT_REGISTER as *mut u32, 1 << 13);
            delay(10000000);
            ptr::write_volatile(OUT_REGISTER as *mut u32, 0 << 13);
            delay(10000000);
        }
    }
}
