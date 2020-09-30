use cryptopals::*;

fn main() {
    let b1 = hex_string_to_bytes("1c0111001f010100061a024b53535009181c");
    let b2 = hex_string_to_bytes("686974207468652062756c6c277320657965");
    let xored = xor_bytes(&b1, &b2);
    println!("Xor is: {}", bytes_to_hex_string(&xored));
}
