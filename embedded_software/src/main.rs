#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod speaker;

use cortex_m_rt::entry;
use microbit::board::Board;
use panic_halt as _;
use speaker::Speaker;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut speaker = Speaker::new(board.speaker_pin, board.PWM0);

    speaker.play_note(1000);

    loop {}
}
