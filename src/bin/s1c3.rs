use cryptopals::*;

fn main() {
    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted = hex_string_to_bytes(encrypted);
    let (key_byte, decrypted) = find_single_character_xor_key(&encrypted);
    println!("Key: {}", key_byte as char);
    println!("Message: {}", String::from_utf8(decrypted).unwrap());
}
