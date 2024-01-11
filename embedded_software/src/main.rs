#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{gpio::*, pwm::*, time::Hertz},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let speaker_pin = board.speaker_pin.into_push_pull_output(Level::High);
    let speaker = Pwm::new(board.PWM0);
    speaker
        .set_output_pin(Channel::C0, speaker_pin.degrade())
        .set_prescaler(Prescaler::Div16)
        .set_period(Hertz(440))
        .set_counter_mode(CounterMode::UpAndDown)
        .enable();

    // Configure 50% duty cycle (volulme = 1000%)
    speaker.set_duty_on_common(speaker.max_duty() / 2);

    loop {}
}
