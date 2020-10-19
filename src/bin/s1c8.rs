use cryptopals::hex_string_to_bytes;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn has_duplicates(bytes: &[u8]) -> bool {
    let mut blocks = bytes.chunks(16).collect::<Vec<_>>();
    let initial_length = blocks.len();

    blocks.sort();
    blocks.dedup();

    blocks.len() != initial_length
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/s1c8.txt")?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| line.ok().map(|line| (line_number, line)))
        .map(|(line_number, line)| (line_number, hex_string_to_bytes(&line)))
        .filter(|(_, bytes)| has_duplicates(&bytes))
        .for_each(|(line_number, bytes)| {
            println!("Line {} is encrypted in ECB mode.", line_number);
            for chunk in bytes.chunks(16) {
                println!("Chunk: {:?}", chunk);
            }
        });

    Ok(())
}
