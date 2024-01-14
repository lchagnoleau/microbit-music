# 🎶 microbit-music 🎶

This project is a small endeavor to learn Rust in an embedded environment using the micro:bit v2 board.

## Project Description

The main objective of this project is to enable playing music from a terminal command like this:
```bash
$ microbit-music -f example/ode_to_joy.txt -d /dev/ttyACM0
```

I cut the project in different steps:

Step 1: Play a single note in the micro:bit

Step 2: Communicate with the micro:bit using serial port

Step 3: Play a sequence of notes from a file

## Embedded Software

The embedded software is written in Rust and uses the [nrf52833-hal](https://crates.io/crates/nrf52833-hal) crate.
To build the software, you need to install the [Rust toolchain](https://rustup.rs/) and other tools:

```bash
$ rustup target add thumbv7em-none-eabihf
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
$ cargo install probe-rs --features cli --forced
```

Then, to build it:
```bash
$ cd embedded_software
$ cargo build --release
```

To flash the micro:bit:
```bash
$ cargo flash --chip nrf52833_xxAA --release
```

## Command Line Interface

The command line interface is also written in Rust. To build it, run:
```bash
$ cd microbit-music
$ cargo build --release
```

To run it:
```bash
$ ./target/release/microbit-music -f example/ode_to_joy.txt -d /dev/ttyACM0

```

## Music Files

The music files are written in a simple format. It is formatted as follows:
```
<note>,<duration_ms>;<note>,<duration_ms>;...
```

Available notes are:
```
'C' (262 Hz)
'D' (294 Hz)
'E' (329 Hz)
'F' (349 Hz)
'G' (392 Hz)
'A' (440 Hz)
'H' (494 Hz)
```
