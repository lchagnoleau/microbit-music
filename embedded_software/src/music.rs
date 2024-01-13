pub fn get_freq(note: char) -> u32 {
    match note {
        'A' => 440,
        'B' => 494,
        'C' => 523,
        'D' => 587,
        'E' => 659,
        'F' => 698,
        'G' => 784,
        _ => 0,
    }
}
