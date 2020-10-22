use cryptopals::aes::aes_128_cbc_decrypt;

use std::fs::File;
use std::io::{BufReader, Read};

use openssl::base64;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/s2c10.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let content = base64::decode_block(&content.replace("\n", "")).unwrap();

    let iv = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let key = "YELLOW SUBMARINE";

    let decrypted = aes_128_cbc_decrypt(&content, key.as_bytes(), iv);

    println!("Decrypted: {}", String::from_utf8(decrypted).unwrap());

    Ok(())
}
