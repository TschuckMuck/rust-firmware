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

// Keyboard constants
const ROWS: u32 = 5;
const COLUMNS: u32 = 14;

// CLOCK constants
const CLOCK_BASE_ADDR: u32 = 0x40000000;
const HIGH_FREQ_CLOCK: u32 = CLOCK_BASE_ADDR + 0x000;
const LOW_FREQ_CLOCK: u32 = CLOCK_BASE_ADDR + 0x008;
const ENABLE_CLOCK: u32 = 0x01;

// UART-specific constants
const UART_BASE_ADDR: u32 = 0x40002000;

const UART_START_RX: u32 = UART_BASE_ADDR + 0x0;
const UART_START_TX: u32 = UART_BASE_ADDR + 0x8;

const UART_PSEL_RXD: u32 = UART_BASE_ADDR + 0x514;
const UART_PSEL_TXD: u32 = UART_BASE_ADDR + 0x50C;

const UART_RXD_REGISTER: u32 = UART_BASE_ADDR + 0x518;
const UART_TXD_REGISTER: u32 = UART_BASE_ADDR + 0x51C;
const TXDRDY: u32 = UART_BASE_ADDR + 0x11C;

const UART_BAUD_RATE: u32 = UART_BASE_ADDR + 0x524;

const UART_ENABLE_ADDR: u32 = UART_BASE_ADDR + 0x500;

const UART_ENABLE: u32 = 0x4;

const UART_9600_BAUDRATE: u32 = 0x00275000;

// GPIO pin-specific constants
// P0
const P0_BASE_ADDR: u32 = 0x50000000;

const P0_PORT_WRITE: u32 = P0_BASE_ADDR + 0x504;
const P0_PORT_READ: u32 = P0_BASE_ADDR + 0x510;

// P1
const P1_BASE_ADDR: u32 = 0x50000300;

const P1_PORT_WRITE: u32 = P1_BASE_ADDR + 0x504;
const P1_PORT_READ: u32 = P1_BASE_ADDR + 0x510;

const CONFIG_OFFSET: u32 = 0x700;

const UART_TX_PIN: u32 = 6;
const UART_TX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_TX_PIN * 4);

const UART_RX_PIN: u32 = 8;
const UART_RX_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (UART_RX_PIN * 4);

const LEDS_PIN: u32 = 13;

// Rows on P0
const ROW1_PIN: u32 = 04;
const ROW2_PIN: u32 = 28;
const ROW3_PIN: u32 = 29;
const ROW4_PIN: u32 = 30;
const ROW5_PIN: u32 = 31;

// Columns on P1
const COLUMN1_PIN: u32 = 01;
const COLUMN2_PIN: u32 = 02;
const COLUMN3_PIN: u32 = 03;
const COLUMN4_PIN: u32 = 04;
const COLUMN5_PIN: u32 = 05;
const COLUMN6_PIN: u32 = 06;
const COLUMN7_PIN: u32 = 07;
const COLUMN8_PIN: u32 = 08;
const COLUMN9_PIN: u32 = 10;
const COLUMN10_PIN: u32 = 11;
const COLUMN11_PIN: u32 = 12;
const COLUMN12_PIN: u32 = 13;
const COLUMN13_PIN: u32 = 14;
const COLUMN14_PIN: u32 = 15;

const ROW1_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (ROW1_PIN * 4);
const ROW2_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (ROW2_PIN * 4);
const ROW3_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (ROW3_PIN * 4);
const ROW4_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (ROW4_PIN * 4);
const ROW5_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (ROW5_PIN * 4);

const COLUMN1_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN1_PIN * 4);
const COLUMN2_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN2_PIN * 4);
const COLUMN3_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN3_PIN * 4);
const COLUMN4_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN4_PIN * 4);
const COLUMN5_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN5_PIN * 4);
const COLUMN6_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN6_PIN * 4);
const COLUMN7_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN7_PIN * 4);
const COLUMN8_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN8_PIN * 4);
const COLUMN9_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN9_PIN * 4);
const COLUMN10_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN10_PIN * 4);
const COLUMN11_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN11_PIN * 4);
const COLUMN12_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN12_PIN * 4);
const COLUMN13_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN13_PIN * 4);
const COLUMN14_CONFIG: u32 = P1_BASE_ADDR + CONFIG_OFFSET + (COLUMN14_PIN * 4);

const LEDS_CONFIG: u32 = P0_BASE_ADDR + CONFIG_OFFSET + (LEDS_PIN * 4);

const ENABLE_INPUT: u32 = 0x0;
const ENABLE_OUTPUT: u32 = 0x3;

const PULL_DOWN: u32 = 0x4;
const PULL_UP: u32 = 0xC;

const ROW_PINS: [u32; ROWS as usize] = [ROW1_PIN, ROW2_PIN, ROW3_PIN, ROW4_PIN, ROW5_PIN];

const COLUMN_PINS: [u32; COLUMNS as usize] = [
    COLUMN1_PIN,
    COLUMN2_PIN,
    COLUMN3_PIN,
    COLUMN4_PIN,
    COLUMN5_PIN,
    COLUMN6_PIN,
    COLUMN7_PIN,
    COLUMN8_PIN,
    COLUMN9_PIN,
    COLUMN10_PIN,
    COLUMN11_PIN,
    COLUMN12_PIN,
    COLUMN13_PIN,
    COLUMN14_PIN,
];

static mut keys: [u8; (COLUMNS * ROWS) as usize] = [0; (COLUMNS * ROWS) as usize];

const KEY_MAP: [u8; (COLUMNS * ROWS) as usize] = [
    0x1B, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x2D, 0x3D, 0x08, 0x09, 0x71,
    0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6F, 0x70, 0x5B, 0x5D, 0x5C,
    //  CAPS                                                                    NONE
    0x00, 0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6A, 0x6B, 0x6C, 0x3B, 0x27, 0x00, 0x0D,
    //  SHFT  NONE                                                              NONE  SHFT
    0x00, 0x00, 0x7A, 0x78, 0x63, 0x76, 0x62, 0x6E, 0x6D, 0x2C, 0x2E, 0x2F, 0x00, 0x00,
    //  RCTL  WNDW  PAGE  NONE  NONE  SPAC  NONE  NONE  NONE  NONE  LALT    FN  RALT  LCTL
    0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const TIMER_BASE_ADDR: u32 = 0x40008000;
const TIMER_32BIT_BITMODE: u32 = 0x03;
const TIMER_TIMER_MODE: u32 = 0x00;
const TIMER_PRESCALE_VALUE: u32 = 0x00;
const ENABLE_TIMER: u32 = 0x01;
const DISABLE_TIMER: u32 = 0x01;
const CLEAR_TIMER_AFTER_TRIGGER: u32 = 0x01;

const CLOCK_START_OSCILLATOR: u32 = CLOCK_BASE_ADDR + 0x000;
const TIMER_BIT_MODE: u32 = TIMER_BASE_ADDR + 0x508;
const TIMER_MODE_REGISTER: u32 = TIMER_BASE_ADDR + 0x504;
const TIMER_PRESCALER_REGISTER: u32 = TIMER_BASE_ADDR + 0x510;
const TIMER_INTERRUPT: u32 = TIMER_BASE_ADDR + 0x304;
const TIMER_TIMEOUT: u32 = TIMER_BASE_ADDR + 0x540;
const TIMER_CONFIG_TRIGGER: u32 = TIMER_BASE_ADDR + 0x200;
const START_ZE_TIMER: u32 = TIMER_BASE_ADDR + 0x000;
const STOP_ZE_TIMER: u32 = TIMER_BASE_ADDR + 0x004;
const TIMER_TRIGGERED: u32 = TIMER_BASE_ADDR + 0x140;
const CLEAR_TIMER: u32 = TIMER_BASE_ADDR + 0x00C;
const TIMER_DISABLE_INTERRUPT: u32 = TIMER_BASE_ADDR + 0x308;

#[inline(never)]
fn delay(us: u32) {
    unsafe {
        // init clock
        ptr::write_volatile(CLOCK_START_OSCILLATOR as *mut u32, ENABLE_CLOCK);

        // init timer
        ptr::write_volatile(TIMER_BIT_MODE as *mut u32, TIMER_32BIT_BITMODE);
        ptr::write_volatile(TIMER_MODE_REGISTER as *mut u32, TIMER_TIMER_MODE);
        ptr::write_volatile(TIMER_PRESCALER_REGISTER as *mut u32, TIMER_PRESCALE_VALUE);

        // enable interrupts
        ptr::write_volatile(TIMER_INTERRUPT as *mut u32, 0x3F << 16);

        ptr::write_volatile(TIMER_TIMEOUT as *mut u32, us);
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

        // enable clockos
        ptr::write_volatile(HIGH_FREQ_CLOCK as *mut u32, ENABLE_CLOCK);
        ptr::write_volatile(LOW_FREQ_CLOCK as *mut u32, ENABLE_CLOCK);

        ptr::write_volatile(ROW1_CONFIG as *mut u32, ENABLE_INPUT | PULL_DOWN);
        ptr::write_volatile(ROW2_CONFIG as *mut u32, ENABLE_INPUT | PULL_DOWN);
        ptr::write_volatile(ROW3_CONFIG as *mut u32, ENABLE_INPUT | PULL_DOWN);
        ptr::write_volatile(ROW4_CONFIG as *mut u32, ENABLE_INPUT | PULL_DOWN);
        ptr::write_volatile(ROW5_CONFIG as *mut u32, ENABLE_INPUT | PULL_DOWN);

        ptr::write_volatile(COLUMN1_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN2_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN3_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN4_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN5_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN6_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN7_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN8_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN9_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN10_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN11_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN12_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN13_CONFIG as *mut u32, ENABLE_OUTPUT);
        ptr::write_volatile(COLUMN14_CONFIG as *mut u32, ENABLE_OUTPUT);

        ptr::write_volatile(LEDS_CONFIG as *mut u32, ENABLE_OUTPUT);

        ptr::write_volatile(P0_PORT_WRITE as *mut u32, 1 << LEDS_PIN);
    }
    loop {
        unsafe {
            for column_index in 0..COLUMNS {
                let COLUMN_MASK = 1 << COLUMN_PINS[column_index as usize];
                ptr::write_volatile(P1_PORT_WRITE as *mut u32, COLUMN_MASK);
                // delay 1 us between column write and row read to allow the voltage on the line to
                // settle
                delay(1);
                let ROW_DATA = ptr::read_volatile(P0_PORT_READ as *mut u32);
                for row_index in 0..ROWS {
                    let key_index = (row_index * COLUMNS + column_index) as usize;
                    let row = (ROW_DATA >> ROW_PINS[row_index as usize]) & 1;
                    let prev_key_value = keys[key_index];
                    keys[key_index] = keys[key_index] << 1 | row as u8;
                    if keys[key_index] == 0xFF && keys[key_index] != prev_key_value {
                        let key_data = KEY_MAP[key_index];
                        ptr::write_volatile(UART_TXD_REGISTER as *mut u32, key_data as u32);
                    }
                }
            }
        }
    }
}
