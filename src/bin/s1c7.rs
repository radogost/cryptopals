use std::fs::File;
use std::io::{BufReader, Read};

use openssl::base64;
use openssl::symm::{decrypt, Cipher};

fn main() -> std::io::Result<()> {
    let file = File::open("./data/s1c7.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let content = base64::decode_block(&content.replace("\n", "")).unwrap();

    let key = "YELLOW SUBMARINE";

    let decrypted = decrypt(Cipher::aes_128_ecb(), key.as_bytes(), None, &content).unwrap();

    println!("Decrypted: {}", String::from_utf8(decrypted).unwrap());

    Ok(())
}
