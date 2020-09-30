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
