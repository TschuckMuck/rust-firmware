//! This application configures a uart which will echo all data received via rx to tx.
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

// UART-specific constants
const UART_BASE_ADDR: u32 = 0x40002000;

const UART_START_RX: u32 = UART_BASE_ADDR + 0x0;
const UART_START_TX: u32 = UART_BASE_ADDR + 0x8;

const UART_PSEL_RXD: u32 = UART_BASE_ADDR + 0x514;
const UART_PSEL_TXD: u32 = UART_BASE_ADDR + 0x50C;

const UART_RXD_REGISTER: u32 = UART_BASE_ADDR + 0x518;
const UART_TXD_REGISTER: u32 = UART_BASE_ADDR + 0x51C;

const UART_BAUD_RATE: u32 = UART_BASE_ADDR + 0x524;

const UART_ENABLE_ADDR: u32 = UART_BASE_ADDR + 0x500;

const UART_ENABLE: u32 = 0x4;

const UART_9600_BAUDRATE: u32 = 0x00275000;

// GPIO pin-specific constants
const P0_BASE_ADDR: u32 = 0x50000000;
const CONFIG_OFFSET: u32 = 0x700;

const UART_TX_PIN: u32 = 6;
const UART_TX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_TX_PIN * 4);

const UART_RX_PIN: u32 = 8;
const UART_RX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_RX_PIN * 4);

const ENABLE_INPUT: u32 = 0x0;
const ENABLE_OUTPUT: u32 = 0x3;

// TODO: Setup peripheral oscillator clock for UART so that UART is stable and not lossy
#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    unsafe {
        ptr::write_volatile(UART_TX_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(UART_RX_CONFIG as *mut u32, ENABLE_INPUT);

        ptr::write_volatile(UART_BAUD_RATE as *mut u32, UART_9600_BAUDRATE);
        // set up UART PSEL RXD/TXD
        ptr::write_volatile(UART_PSEL_RXD as *mut u32, UART_RX_PIN);
        ptr::write_volatile(UART_PSEL_TXD as *mut u32, UART_TX_PIN);
        // enable UART
        ptr::write_volatile(UART_ENABLE_ADDR as *mut u32, UART_ENABLE);
        // enable UART RX/TX
        ptr::write_volatile(UART_START_RX as *mut u32, 0x1);
        ptr::write_volatile(UART_START_TX as *mut u32, 0x1);
    }
    loop {
        unsafe {
            let uart_char = ptr::read_volatile(UART_RXD_REGISTER as *mut u32);
            if uart_char != 0 {
                ptr::write_volatile(UART_TXD_REGISTER as *mut u32, uart_char);
            }
        }
    }
}
