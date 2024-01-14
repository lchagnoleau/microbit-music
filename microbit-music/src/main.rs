use clap::Parser;
use serialport;
use std::{io, time::Duration};

struct Note {
    name: char,
    tempo: u32,
}

fn parse_music_file(file_path: &str) -> Result<Vec<Note>, io::Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut music = Vec::new();

    for line in file.lines() {
        for note in line.split(';') {
            let mut n = note.split(',');
            let name = n.next().unwrap().chars().next().unwrap();
            let tempo = n.next().unwrap().parse::<u32>().unwrap();

            music.push(Note { name, tempo });
        }
    }

    Ok(music)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the music file
    #[arg(short, long)]
    file_path: String,

    /// Device path
    #[arg(short, long)]
    device_path: String,
}

fn main() {
    let args = Args::parse();

    let mut port = serialport::new(args.device_path, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    if let Ok(music) = parse_music_file(args.file_path.as_str()) {
        for note in music {
            port.write(note.name.to_string().as_bytes())
                .expect("Write failed!");
            std::thread::sleep(Duration::from_millis(note.tempo as u64));
        }

        port.write("0".as_bytes()).expect("Write failed!");
    } else {
        println!("Cannot Open file: {}", args.file_path);
    }
}
