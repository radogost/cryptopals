use cryptopals::*;

fn main() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\n\
    I go crazy when I hear a cymbal";
    let key = "ICE";
    let encrypted = repeating_xor(plaintext.as_bytes(), key);
    println!("Encrypted: {}", bytes_to_hex_string(&encrypted));
}
