use microbit::hal::{gpio::*, pwm::*, time::Hertz};
use microbit::pac;
pub struct Speaker {
    speaker: Pwm<pac::PWM0>,
}

impl Speaker {
    pub fn new(speaker_pin: p0::P0_00<Disconnected>, pwm_instance: pac::PWM0) -> Self {
        let speaker = Pwm::new(pwm_instance);
        speaker
            .set_output_pin(Channel::C0, speaker_pin.into_push_pull_output(Level::High).degrade())
            .set_prescaler(Prescaler::Div16)
            .set_counter_mode(CounterMode::UpAndDown)
            .enable();

        Self { speaker }
    }

    pub fn play_note(&mut self, frequency: u32) {
        self.speaker.set_period(Hertz(frequency));
        self.speaker.set_duty_on_common(self.speaker.max_duty() / 2);
    }
}
