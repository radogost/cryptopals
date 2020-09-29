use cryptopals::*;

fn xor_bytes(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1.into_iter().zip(b2).map(|(a, b)| a ^ b).collect()
}

fn main() {
    let b1 = hex_string_to_bytes("1c0111001f010100061a024b53535009181c");
    let b2 = hex_string_to_bytes("686974207468652062756c6c277320657965");
    let xored = xor_bytes(&b1, &b2);
    println!("Xor is: {}", bytes_to_hex_string(&xored));
}
