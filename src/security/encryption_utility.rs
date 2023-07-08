// src/security/encryption_utility.rs
use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use std::io::Error;

pub struct EncryptionUtility {
    key: Vec<u8>,
    cipher: Cipher,
}

impl EncryptionUtility {
    pub fn new(key: Vec<u8>) -> Self {
        Self {
            key,
            cipher: Cipher::aes_256_cbc(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        let mut iv = [0u8; 16]; // 128 bits
        rand::thread_rng().fill_bytes(&mut iv);

        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, &self.key, Some(&iv))?;
        crypter.pad(true);

        let mut ciphertext = vec![0; iv.len() + plaintext.len() + self.cipher.block_size()];
        ciphertext[..iv.len()].copy_from_slice(&iv);
        let count = crypter.update(plaintext, &mut ciphertext[iv.len()..])?;
        let rest = crypter.finalize(&mut ciphertext[iv.len() + count..])?;
        ciphertext.truncate(iv.len() + count + rest);

        Ok(ciphertext)
    }

    // pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    //     let (iv, ciphertext) = ciphertext.split_at(16);

    //     let mut crypter = Crypter::new(self.cipher, Mode::Decrypt, &self.key, Some(iv))?;
    //     crypter.pad(true);

    //     let mut plaintext = vec![0; ciphertext.len() + self.cipher.block_size()];
    //     let count = crypter.update(ciphertext, &mut plaintext)?;
    //     let rest = crypter.finalize(&mut plaintext[count..])?;
    //     plaintext.truncate(count + rest);

    //     Ok(plaintext)
    // }
}
