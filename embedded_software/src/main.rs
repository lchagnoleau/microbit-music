#![deny(unsafe_code)]
#![no_main]
#![no_std]

// mod speaker;
mod serial;
mod music;

use cortex_m_rt::entry;
use hal::gpio::{p0, p1};
use nrf52833_hal as hal;
use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
// use speaker::Speaker;
use serial::Serial;
use music::get_freq;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting up!");

    let pac = hal::pac::Peripherals::take().unwrap();
    let p0 = p0::Parts::new(pac.P0);
    let p1 = p1::Parts::new(pac.P1);

    let mut serial = Serial::new(pac.UARTE0, p0.p0_06, p1.p1_08);

    serial.write(b"Hello, world!\r\n");

    let mut note_freq: u32 = 0;
    loop {
        note_freq = get_freq(serial.read());
        rprintln!("new note: {}", note_freq);
    }
}
