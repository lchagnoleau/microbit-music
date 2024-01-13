use core::fmt::Write;

use nrf52833_hal::{
    gpio::{p0::P0_06, p1::P1_08, Disconnected, Level::High},
    pac::UARTE0,
    uarte,
    uarte::Uarte,
};

pub struct Serial {
    serial: Uarte<UARTE0>,
}

impl Serial {
    pub fn new(
        uart_instance: UARTE0,
        tx_pin: P0_06<Disconnected>,
        rx_pin: P1_08<Disconnected>,
    ) -> Self {
        let txd = tx_pin.into_push_pull_output(High).degrade();
        let rxd = rx_pin.into_floating_input().degrade();

        let uart_pins = uarte::Pins {
            txd,
            rxd,
            cts: None,
            rts: None,
        };

        let serial = Uarte::new(
            uart_instance,
            uart_pins,
            uarte::Parity::EXCLUDED,
            uarte::Baudrate::BAUD115200,
        );

        Self { serial }
    }

    pub fn write(&mut self, data: &[u8]) {
        if let Ok(s) = core::str::from_utf8(data) {
            self.serial.write_str(s).unwrap();
        }
    }
}
