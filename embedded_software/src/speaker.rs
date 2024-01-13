use nrf52833_hal::{
    gpio::{p0::P0_00, Disconnected, Level::High},
    pac::PWM0,
    pwm::{Channel, CounterMode, Prescaler, Pwm},
    time::Hertz,
};
pub struct Speaker {
    speaker: Pwm<PWM0>,
}

impl Speaker {
    pub fn new(pwm_instance: PWM0, speaker_pin: P0_00<Disconnected>) -> Self {
        let speaker = Pwm::new(pwm_instance);
        speaker
            .set_output_pin(Channel::C0, speaker_pin.into_push_pull_output(High).degrade())
            .set_prescaler(Prescaler::Div16)
            .set_counter_mode(CounterMode::UpAndDown)
            .enable();

        Self { speaker }
    }

    pub fn play_note(&mut self, frequency: u32) {
        if frequency == 0 {
            self.speaker.disable();
            return;
        }
        self.speaker.set_period(Hertz(frequency));
        self.speaker.set_duty_on_common(self.speaker.max_duty() / 2);
    }
}
