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

const P0_BASE_ADDR: u32   = 0x50000000;

const OUT_REG_OFFSET: u32 = 0x504;
const IN_REG_OFFSET: u32  = 0x510;

const P0_PORT_WRITE: u32  = P0_BASE_ADDR + OUT_REG_OFFSET;
const P0_PORT_READ: u32   = P0_BASE_ADDR + IN_REG_OFFSET;

const CONFIG_OFFSET: u32  = 0x700;

const BUTTON1_PIN: u32    = 11;
const BUTTON2_PIN: u32    = 12;
const BUTTON3_PIN: u32    = 24;
const BUTTON4_PIN: u32    = 25;

const LED1_PIN: u32       = 13;
const LED2_PIN: u32       = 14;
const LED3_PIN: u32       = 15;
const LED4_PIN: u32       = 16;

// 4 bytes per pin for configuration
const BUTTON1_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (BUTTON1_PIN * 4);
const BUTTON2_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (BUTTON2_PIN * 4);
const BUTTON3_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (BUTTON3_PIN * 4);
const BUTTON4_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (BUTTON4_PIN * 4);

const LED1_CONFIG: u32    = P0_BASE_ADDR + CONFIG_OFFSET + (LED1_PIN * 4);
const LED2_CONFIG: u32    = P0_BASE_ADDR + CONFIG_OFFSET + (LED2_PIN * 4);
const LED3_CONFIG: u32    = P0_BASE_ADDR + CONFIG_OFFSET + (LED3_PIN * 4);
const LED4_CONFIG: u32    = P0_BASE_ADDR + CONFIG_OFFSET + (LED4_PIN * 4);

// GPIO PIN constants
const DISABLE_PIN: u32    = 0x2;
const ENABLE_INPUT: u32   = 0x0;
const ENABLE_OUTPUT: u32  = 0x3;

const DISABLE_PULL: u32   = 0x0;
const PULL_DOWN: u32      = 0x4;
const PULL_UP: u32        = 0xC;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    unsafe {
        // Set LED pins to 1 to turn them off before enabling LEDs
        let leds = 1 << LED1_PIN | 1 << LED2_PIN | 1 << LED3_PIN | 1 << LED4_PIN;
        ptr::write_volatile(P0_PORT_WRITE as *mut u32, leds);

        ptr::write_volatile(LED1_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(LED2_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(LED3_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(LED4_CONFIG as *mut u32, ENABLE_OUTPUT);

        ptr::write_volatile(BUTTON1_CONFIG as *mut u32, ENABLE_INPUT | PULL_UP);
        ptr::write_volatile(BUTTON2_CONFIG as *mut u32, ENABLE_INPUT | PULL_UP);
        ptr::write_volatile(BUTTON3_CONFIG as *mut u32, ENABLE_INPUT | PULL_UP);
        ptr::write_volatile(BUTTON4_CONFIG as *mut u32, ENABLE_INPUT | PULL_UP);
    }
    loop {
        unsafe {
            let port_data = ptr::read_volatile(P0_PORT_READ as *mut u32);
            let led1 = (port_data >> BUTTON1_PIN) & 0x1;
            let led2 = (port_data >> BUTTON2_PIN) & 0x1;
            let led3 = (port_data >> BUTTON3_PIN) & 0x1;
            let led4 = (port_data >> BUTTON4_PIN) & 0x1;
            let leds = led1 << LED1_PIN | led2 << LED2_PIN | led3 << LED3_PIN | led4 << LED4_PIN;
            ptr::write_volatile(P0_PORT_WRITE as *mut u32, leds);
        }
    }
}
