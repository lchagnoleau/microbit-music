#![deny(unsafe_code)]
#![no_main]
#![no_std]

// mod speaker;
// mod uart;

use core::fmt::Write;
use cortex_m_rt::entry;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52833_hal as hal;
use panic_halt as _;
// use speaker::Speaker;
// use uart::Uart;

#[entry]
fn main() -> ! {
    let pac = hal::pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(pac.P0);
    let p1 = gpio::p1::Parts::new(pac.P1);

    let txd = p0.p0_06.into_push_pull_output(gpio::Level::High).degrade();
    let rxd = p1.p1_08.into_floating_input().degrade();

    let uart_pins = uarte::Pins {
        txd,
        rxd,
        cts: None,
        rts: None,
    };

    let mut serial = Uarte::new(
        pac.UARTE0,
        uart_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    write!(serial, "Hello, World!\r\n").unwrap();

    loop {}
}
