use cryptopals::*;

use std::fs::File;
use std::io::{BufReader, Read};

fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2)
        .map(|(a, b)| (a ^ b).count_ones())
        .fold(0, |a, b| a + b)
}

fn get_keysize_candidates(bytes: &[u8], max_candidates: u8) -> Vec<u8> {
    let mut size_hamming_distance_pairs = vec![];

    for size_candidate in 2..40 {
        let mut chunks = bytes.chunks(size_candidate);
        let chunk1 = chunks.next().unwrap();
        let chunk2 = chunks.next().unwrap();
        let chunk3 = chunks.next().unwrap();
        let chunk4 = chunks.next().unwrap();
        let distance = hamming_distance(chunk1, chunk2) + hamming_distance(chunk3, chunk4);
        size_hamming_distance_pairs.push((
            size_candidate,
            distance as f64 / (2.0 * size_candidate as f64),
        ));
    }

    size_hamming_distance_pairs.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

    size_hamming_distance_pairs[0..(max_candidates as usize)]
        .iter()
        .map(|(size, _)| *size as u8)
        .collect()
}

fn transpose_blocks(bytes: &[u8], keysize: u8) -> Vec<Vec<u8>> {
    let mut blocks = vec![Vec::new(); keysize as usize];
    for chunk in bytes.chunks(keysize as usize) {
        for (block, byte) in chunk.iter().enumerate() {
            blocks[block].push(*byte);
        }
    }
    blocks
}

fn get_key_candidate(blocks: &[Vec<u8>]) -> String {
    let mut key = String::with_capacity(blocks[0].len());

    for block in blocks {
        let (c, _) = find_single_character_xor_key(block);
        key.push(c);
    }

    key
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/s1c6.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let decoded_content = decode_base64(&content);

    let keysize_candidates = get_keysize_candidates(&decoded_content, 10);

    let mut key_candidates = vec![];

    for keysize_candidate in keysize_candidates {
        let transposed_blocks = transpose_blocks(&decoded_content, keysize_candidate);
        key_candidates.push(get_key_candidate(&transposed_blocks));
    }

    let mut key = String::new();
    let mut decrypted_message = String::new();
    let mut highest_english_score = 0.0;

    for key_candidate in key_candidates {
        let decrypted = repeating_xor(&decoded_content, &key_candidate);
        let score = english_score(&decrypted);
        if score > highest_english_score {
            if let Ok(decrypted) = String::from_utf8(decrypted) {
                highest_english_score = score;
                decrypted_message = decrypted;
                key = key_candidate;
            }
        }
    }

    println!("Key: {}, Message: {}", key, decrypted_message);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hamming_distance() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let distance = hamming_distance(s1.as_bytes(), s2.as_bytes());
        assert_eq!(distance, 37);
    }
}
