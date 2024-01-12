#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod speaker;
mod uart;

use cortex_m_rt::entry;
use microbit::board::Board;
use panic_halt as _;
use speaker::Speaker;
use uart::Uart;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut uart = Uart::new(board.UARTE0, board.uart.into());
    let mut speaker = Speaker::new(board.speaker_pin, board.PWM0);

    speaker.play_note(0);

    uart.write(b"Hello, world!\r\n");

    loop {
        uart.write(b"Hello, world!\r\n");
    }
}
