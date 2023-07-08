// src/security/encrypt.rs
use crate::security::encryption_utility::EncryptionUtility;
use hex;
use rand::RngCore;
use std::io::Error;

pub struct Encryptor {
    psk_session: EncryptionUtility,
}

impl Encryptor {
    pub fn new(key: Option<Vec<u8>>) -> Self {
        let key = match key {
            Some(k) => k,
            None => {
                let mut new_key = vec![0u8; 32]; // 256 bits
                rand::thread_rng().fill_bytes(&mut new_key);
                println!("Generated PSK: {:?}", hex::encode(&new_key));
                new_key
            }
        };

        let psk_session = EncryptionUtility::new(key.clone());

        Self { psk_session }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        self.psk_session.encrypt(plaintext)
    }
}
