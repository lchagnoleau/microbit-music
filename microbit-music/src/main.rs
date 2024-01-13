use serialport;
use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115_200)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    let note = "A";

    port.write(note.as_bytes()).expect("Write failed!");
}
