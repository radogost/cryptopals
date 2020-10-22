pub mod aes;

pub fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    if s.len() % 2 != 0 {
        unimplemented!("Supports only hex strings with even length so far");
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:x}", b)).collect()
}

pub fn xor_bytes(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1.into_iter().zip(b2).map(|(a, b)| a ^ b).collect()
}

// source: http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
static ENGLISH_LETTER_FREQUENCIES: [(char, f64); 27] = [
    ('a', 6.53),
    ('b', 1.25),
    ('c', 2.23),
    ('d', 3.28),
    ('e', 10.26),
    ('f', 1.98),
    ('g', 1.62),
    ('h', 4.97),
    ('i', 5.66),
    ('j', 0.09),
    ('k', 0.56),
    ('l', 3.31),
    ('m', 2.02),
    ('n', 5.71),
    ('o', 6.15),
    ('p', 1.50),
    ('q', 0.08),
    ('r', 4.98),
    ('s', 5.31),
    ('t', 7.51),
    ('u', 2.27),
    ('v', 0.79),
    ('w', 1.70),
    ('x', 0.14),
    ('y', 1.42),
    ('z', 0.05),
    (' ', 18.28),
];

/// Bhattacharyya coefficient (compares to english letter frequencies)
pub fn english_score(message: &[u8]) -> f64 {
    let mut letter_count = 0.0;
    let mut single_letter_count = [0.0; 27];

    for ch in message.iter() {
        if ch.is_ascii_alphabetic() {
            letter_count += 1.0;
            single_letter_count[(ch.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
        } else if (*ch as char) == ' ' {
            letter_count += 1.0;
            single_letter_count[26] += 1.0;
        }
    }

    let mut score = 0.0;
    for (i, count) in single_letter_count.iter().enumerate() {
        let expected_count = letter_count * ENGLISH_LETTER_FREQUENCIES[i].1;
        score += (count * expected_count).sqrt();
    }

    score
}

pub fn find_single_character_xor_key(message: &[u8]) -> (char, Vec<u8>) {
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

    (key_byte as char, decrypted)
}

pub fn repeating_xor(msg: &[u8], key: &str) -> Vec<u8> {
    let key = key.chars().cycle();
    msg.iter().zip(key).map(|(a, b)| (*a) ^ (b as u8)).collect()
}
