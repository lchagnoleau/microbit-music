use microbit::hal::uarte::*;
use microbit::pac;

pub struct Uart {
    uart: Uarte<pac::UARTE0>,
}

impl Uart {
    pub fn new(uart_instance: pac::UARTE0, uart_pin: Pins) -> Self {
        let uart = Uarte::new(
            uart_instance,
            uart_pin,
            Parity::EXCLUDED,
            Baudrate::BAUD115200);

        Self { uart }
    }

    pub fn write(&mut self, data: &[u8]) {
        let _ = self.uart.write(data);
    }
}
