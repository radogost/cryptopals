use cryptopals::*;

fn repeating_xor(msg: &str, key: &str) -> Vec<u8> {
    let key = key.chars().cycle();
    msg.chars()
        .zip(key)
        .map(|(a, b)| (a as u8) ^ (b as u8))
        .collect()
}

fn main() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\n\
    I go crazy when I hear a cymbal";
    let key = "ICE";
    let encrypted = repeating_xor(plaintext, key);
    println!("Encrypted: {}", bytes_to_hex_string(&encrypted));
}
