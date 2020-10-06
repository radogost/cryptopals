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

pub fn decode_base64(base64: &str) -> Vec<u8> {
    fn base64char_to_bits(c: char) -> u8 {
        match c {
            'A'..='Z' => c as u8 - b'A',
            'a'..='z' => c as u8 - b'a' + 26,
            '0'..='9' => c as u8 - b'0' + 52,
            '+' => 62,
            '/' => 63,
            '=' => 0,
            _ => unreachable!(),
        }
    }

    base64
        .replace("\n", "")
        .as_bytes()
        .chunks(4)
        .map(|chunk| {
            let c1 = chunk[0];
            let c2 = chunk[1];
            let c3 = chunk[2];
            let c4 = chunk[3];

            let b1 = base64char_to_bits(c1 as char);
            let b2 = base64char_to_bits(c2 as char);
            let b3 = base64char_to_bits(c3 as char);
            let b4 = base64char_to_bits(c4 as char);

            let mut decoded_chunk = vec![(b1 << 2) | (b2 >> 4)];

            if c3 as char != '=' {
                decoded_chunk.push(((b2 & 0b1111) << 4) | b3 >> 2);
            }

            if c4 as char != '=' {
                decoded_chunk.push((b3 << 6) | b4);
            }

            decoded_chunk
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_base64() {
        let s = "TWFu";
        let decoded = decode_base64(&s);
        let expected = vec![0b01001101, 0b01100001, 0b01101110];
        assert_eq!(decoded, expected);

        let s = "TWE=";
        let decoded = decode_base64(&s);
        let expected = vec![0b01001101, 0b01100001];
        assert_eq!(decoded, expected);

        let s = "TQ==";
        let decoded = decode_base64(&s);
        let expected = vec![0b01001101];
        assert_eq!(decoded, expected);
    }
}
