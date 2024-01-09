# 🎶 microbit-music 🎶

This project is a small endeavor to learn Rust in an embedded environment using the micro:bit v2 board.

## Project Description

The main objective of this project is to enable playing music from a terminal command like this:
```bash
$ microbit-music play "C,10 D,45 E,1"
```
Play a note for a given duration. The notes are specified as a comma separated list of note,duration pairs. The notes are specified as a letter from A to G, optionally followed by a sharp (#) or flat (b) sign, optionally followed by a number from 0 to 4 to specify the octave. The duration is specified as a number followed by a unit, where the unit is one of s (second), ms (millisecond), us (microsecond), or ns (nanosecond). The default unit is ms.

The program also supports using a music file:
```bash
$ microbit-music play -f music.txt
```

I cut the project in different steps:

Step 1: Play a single note in the micro:bit

Step 2: Communicate with the micro:bit using serial port

Step 3: Play a single note from the terminal

Step 4: Play a sequence of notes from the terminal

Step 5: Play a sequence of notes from a file
