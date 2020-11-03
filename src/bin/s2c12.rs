use cryptopals::aes::{aes_128_ecb_encrypt, detect_encryption_mode};

#[macro_use]
extern crate lazy_static;

use openssl::base64::decode_block;
use rand::{thread_rng, Rng};

static UNKNOWN_STRING: &'static str = "\
    Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
    YnkK";

lazy_static! {
    static ref KEY: [u8; 16] = {
        let mut rng = thread_rng();
        rng.gen()
    };
}

fn encrypt(data: &[u8]) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend_from_slice(data);
    bytes.extend_from_slice(&decode_block(UNKNOWN_STRING).unwrap());

    aes_128_ecb_encrypt(&bytes, &*KEY)
}

fn find_block_size() -> usize {
    let initial = encrypt("".as_bytes()).len();
    let mut current = initial;

    let mut data = String::new();
    while initial == current {
        data.push('a');
        current = encrypt(data.as_bytes()).len();
    }

    current - initial
}

struct Decrypter {
    short_block: Vec<u8>,
    decrypt_block: Vec<u8>,
    block_size: usize,
    offset: usize,
}

impl Decrypter {
    fn new(block_size: usize) -> Self {
        Self {
            short_block: Vec::with_capacity(block_size - 1),
            decrypt_block: vec![0; block_size - 1],
            block_size,
            offset: 0,
        }
    }
}

impl Iterator for Decrypter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.short_block
            .resize(self.block_size - self.offset % self.block_size, 0);
        let block_offset = self.offset / self.block_size;
        let start = block_offset * self.block_size;
        let end = start + self.block_size;
        let encrypted = &encrypt(&self.short_block)[start..end];

        self.decrypt_block.push(0);
        let matched_byte = (u8::MIN..=u8::MAX).find(|&byte| {
            self.decrypt_block.pop();
            self.decrypt_block.push(byte);
            let candidate = &encrypt(&self.decrypt_block)[..self.block_size];

            candidate == encrypted
        });

        self.decrypt_block.remove(0);
        self.offset += 1;

        matched_byte.map(|byte| byte as char)
    }
}

fn main() {
    let block_size = find_block_size();
    println!("Block size: {}", block_size);

    let data = [0; 48];
    let mode = detect_encryption_mode(&encrypt(&data));
    println!("Encrypted as: {}", mode);

    let decrypter = Decrypter::new(block_size);
    let decrypted = decrypter.collect::<String>();
    println!("Decrypted: {}", decrypted);
}
