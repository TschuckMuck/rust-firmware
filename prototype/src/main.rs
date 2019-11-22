#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;

use core::iter::Enumerate;
use core::ptr;

// Keyboard constants
const ROWS: usize = 5;
const COLUMNS: usize = 14;

// CLOCK constants
const CLOCK_BASE_ADDR: u32 = 0x40000000;
const HIGH_FREQ_CLOCK: u32 = CLOCK_BASE_ADDR + 0x000;
const LOW_FREQ_CLOCK: u32 = CLOCK_BASE_ADDR + 0x008;
const ENABLE_CLOCK: u32 = 0x01;

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

use hal::gpio::{Input, Output};
use hal::io::{Read, Write};
use hal::uart::BaudRate::Baud115200;
use hal::uart::{BaudRate, Configuration, Configure, Parity, StopBits};
use nordic::nrf52::nrf52840;
use nordic::nrf52::nrf52840::{GpIo, Pin, Port, Uart};

// TODO: Setup peripheral oscillator clock for UART so that UART is stable and not lossy
#[entry]
fn main() -> ! {
    let mut keys: [u8; (COLUMNS * ROWS) as usize] = [0; (COLUMNS * ROWS) as usize];
    let mut uart = Uart::new(GpIo::new(Port::P0, Pin::P8), GpIo::new(Port::P0, Pin::P6));
    uart.configure(Configuration {
        baud_rate: Baud115200,
        stop_bits: StopBits::One,
        parity: Parity::None,
        hw_flow_control: false,
    });

    let backlight: nrf52840::Output = GpIo::new(Port::P0, Pin::P13).into();

    let rows: [nrf52840::Input; ROWS] = [
        GpIo::new(Port::P0, Pin::P4).into(),
        GpIo::new(Port::P0, Pin::P28).into(),
        GpIo::new(Port::P0, Pin::P29).into(),
        GpIo::new(Port::P0, Pin::P30).into(),
        GpIo::new(Port::P0, Pin::P31).into(),
    ];

    let mut columns: [nrf52840::Output; COLUMNS] = [
        GpIo::new(Port::P1, Pin::P1).into(),
        GpIo::new(Port::P1, Pin::P2).into(),
        GpIo::new(Port::P1, Pin::P3).into(),
        GpIo::new(Port::P1, Pin::P4).into(),
        GpIo::new(Port::P1, Pin::P5).into(),
        GpIo::new(Port::P1, Pin::P6).into(),
        GpIo::new(Port::P1, Pin::P7).into(),
        GpIo::new(Port::P1, Pin::P8).into(),
        GpIo::new(Port::P1, Pin::P10).into(),
        GpIo::new(Port::P1, Pin::P11).into(),
        GpIo::new(Port::P1, Pin::P12).into(),
        GpIo::new(Port::P1, Pin::P13).into(),
        GpIo::new(Port::P1, Pin::P14).into(),
        GpIo::new(Port::P1, Pin::P15).into(),
    ];

    uart.enable();

    loop {
        for (c_index, column) in columns.iter_mut().enumerate() {
            column.on();
            // delay 1 us between column write and row read to allow the voltage on the line to settle
            delay(1);
            for (r_index, row) in rows.iter().enumerate() {
                let key_index = r_index * COLUMNS + c_index;
                let prev_key_value: u8 = keys[key_index];
                keys[key_index] = keys[key_index] << 1 | row.read() as u8;
                if keys[key_index] == 0xFF && keys[key_index] != prev_key_value {
                    let key_value = KEY_MAP[key_index];
                    uart.write_byte(key_value);
                }
            }
        }
    }
}
