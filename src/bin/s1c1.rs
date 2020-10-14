use cryptopals::*;

use openssl::base64;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex_string_to_bytes(hex);
    let base64 = base64::encode_block(&bytes);
    println!("Base64 is: {}", base64);
}
