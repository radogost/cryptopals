use cryptopals::*;

use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

fn find_message_in_file(file: &str) -> Result<String> {
    let file = File::open(file)?;
    let file = BufReader::new(file);
    let mut decrypted_bytes = vec![];
    let mut highest_score = 0.0;

    for line in file.lines().filter_map(|line| line.ok()) {
        let encrypted_line = hex_string_to_bytes(&line);
        let (_, candidate) = find_single_character_xor_key(&encrypted_line);
        let score = english_score(&candidate);
        if score > highest_score {
            highest_score = score;
            decrypted_bytes = candidate;
        }
    }

    Ok(String::from_utf8(decrypted_bytes).unwrap())
}
fn main() -> Result<()> {
    let file = "./data/s1c4.txt";
    let decrypted = find_message_in_file(file)?;
    println!("Message: {}", decrypted);
    Ok(())
}
