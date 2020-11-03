use cryptopals::aes::{aes_128_cbc_encrypt, aes_128_ecb_encrypt, detect_encryption_mode};

use rand::{rngs::ThreadRng, thread_rng, Rng};

fn get_random_padding(rng: &mut ThreadRng) -> Vec<u8> {
    let padding_size: u8 = rng.gen_range(5, 10);
    let mut padding = vec![0u8; padding_size as usize];
    for x in padding.iter_mut() {
        *x = rng.gen();
    }

    padding
}

fn encryption_oracle(data: &[u8]) -> Vec<u8> {
    let mut rng = thread_rng();

    let mut bytes = vec![];
    bytes.extend_from_slice(&get_random_padding(&mut rng));
    bytes.extend_from_slice(data);
    bytes.extend_from_slice(&get_random_padding(&mut rng));

    let key: [u8; 16] = rng.gen();
    let coin: f64 = rng.gen_range(0.0, 1.0);

    if coin < 0.5 {
        let iv: [u8; 16] = rng.gen();
        aes_128_cbc_encrypt(&bytes, &key, &iv)
    } else {
        aes_128_ecb_encrypt(&bytes, &key)
    }
}

fn main() {
    let data = [42; 64];

    for _ in 0..10 {
        let encrypted_data = encryption_oracle(&data);
        let mode = detect_encryption_mode(&encrypted_data);
        println!("Data was AES encrypted in {} mode", mode);
    }
}
