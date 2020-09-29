use cryptopals::*;

fn encode_to_base64(bytes: &[u8]) -> String {
    if bytes.len() % 3 != 0 {
        unimplemented!("Supports only bytes which don't produce trailing =");
    }

    fn index_to_base64(index: u8) -> char {
        if index < 26 {
            ('A' as u8 + index) as char
        } else if index < 52 {
            ('a' as u8 + index - 26) as char
        } else if index < 62 {
            ('0' as u8 + index - 52) as char
        } else if index == 62 {
            '+'
        } else {
            '/'
        }
    }

    bytes
        .chunks(3)
        .map(|chunk| {
            let b1 = chunk[0];
            let b2 = chunk[1];
            let b3 = chunk[2];

            vec![
                b1 >> 2,
                ((b1 & 0b11) << 4) | (b2 >> 4),
                ((b2 & 0b1111) << 2) | (b3 >> 6),
                b3 & 0b00111111,
            ]
            .into_iter()
            .map(|b| index_to_base64(b))
        })
        .flatten()
        .collect()
}

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex_string_to_bytes(hex);
    let base64 = encode_to_base64(&bytes);
    println!("Base64 is: {}", base64);
}
