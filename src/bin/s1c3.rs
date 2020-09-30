use cryptopals::*;

// source: http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
static ENGLISH_LETTER_FREQUENCIES: [(char, f64); 26] = [
    ('a', 8.12),
    ('b', 1.49),
    ('c', 2.71),
    ('d', 4.32),
    ('e', 12.02),
    ('f', 2.30),
    ('g', 2.03),
    ('h', 5.92),
    ('i', 7.31),
    ('j', 0.10),
    ('k', 0.69),
    ('l', 3.98),
    ('m', 2.61),
    ('n', 6.95),
    ('o', 7.68),
    ('p', 1.82),
    ('q', 0.11),
    ('r', 6.02),
    ('s', 6.28),
    ('t', 9.10),
    ('u', 2.88),
    ('v', 1.11),
    ('w', 2.09),
    ('x', 0.17),
    ('y', 2.11),
    ('z', 0.07),
];

/// Bhattacharyya coefficient (compares to english letter frequencies)
fn english_score(message: &[u8]) -> f64 {
    let mut letter_count = 0.0;
    let mut single_letter_count = [0.0; 26];

    for ch in message.iter() {
        if ch.is_ascii_alphabetic() {
            letter_count += 1.0;
            single_letter_count[(ch.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
        }
    }

    let mut score = 0.0;
    for (i, count) in single_letter_count.iter().enumerate() {
        let expected_count = letter_count * ENGLISH_LETTER_FREQUENCIES[i].1;
        score += (count * expected_count).sqrt();
    }

    score
}

fn find_single_character_xor_key(message: &[u8]) -> (u8, Vec<u8>) {
    let mut highest_score = 0.0;
    let mut decrypted = vec![];
    let mut key_byte = 0;

    for byte in 0..=255 {
        let key = vec![byte; message.len()];
        let xored = xor_bytes(&message, &key);
        let score = english_score(&xored);
        if score > highest_score {
            key_byte = byte;
            highest_score = score;
            decrypted = xored;
        }
    }

    (key_byte, decrypted)
}

fn main() {
    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted = hex_string_to_bytes(encrypted);
    let (key_byte, decrypted) = find_single_character_xor_key(&encrypted);
    println!("Key: {}", key_byte as char);
    println!("Message: {}", String::from_utf8(decrypted).unwrap());
}
