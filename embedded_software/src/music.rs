pub fn get_freq(note: char) -> u32 {
    match note {
        'C' => 262,
        'D' => 294,
        'E' => 329,
        'F' => 349,
        'G' => 392,
        'A' => 440,
        'H' => 494,
        _ => 0,
    }
}
