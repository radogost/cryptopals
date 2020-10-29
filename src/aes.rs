use crate::xor_bytes;

use openssl::symm::{decrypt, encrypt, Cipher, Crypter, Mode};

pub fn aes_128_cbc_encrypt(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut encrypted = vec![];
    let mut chunk = [0; 32];
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None).unwrap();
    crypter.pad(false);

    let mut cipher = iv;
    for block in bytes.chunks(16) {
        let block = xor_bytes(&cipher, block);
        crypter.update(&block, &mut chunk).unwrap();
        encrypted.extend_from_slice(&chunk[..16]);

        cipher = &chunk[..16];
    }

    encrypted
}

pub fn aes_128_cbc_decrypt(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut decrypted = vec![];
    let mut chunk = [0; 32];
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    crypter.pad(false);

    bytes.chunks(16).fold(iv, |prev_cipher, block| {
        crypter.update(block, &mut chunk).unwrap();
        let decrypted_block = xor_bytes(prev_cipher, &chunk[..16]);
        decrypted.extend_from_slice(&decrypted_block);

        block
    });

    decrypted
}

pub fn aes_128_ecb_encrypt(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key, None, bytes).unwrap()
}

pub fn aes_128_ecb_decrypt(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, bytes).unwrap()
}

#[cfg(test)]
mod tests {

    use super::{aes_128_cbc_decrypt, aes_128_cbc_encrypt};

    #[test]
    fn can_encrypt_and_decrypt() {
        let message = "12345678poiuytreaqswdefrgthyjuki".as_bytes();
        let key = "zxcvbnm,asdfghjk".as_bytes();
        let iv = "qwertyuiasdfghjk".as_bytes();

        let encrypted = aes_128_cbc_encrypt(message, key, iv);
        let decrypted = aes_128_cbc_decrypt(&encrypted, key, iv);

        assert_eq!(decrypted, message);
    }
}
