// src/p2p/peer.rs
use crate::security::encrypt::Encryptor;
use crate::transport::sender::Sender;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub struct Peer {
    encryptor: Encryptor,
}

impl Peer {
    pub fn new(key: Option<Vec<u8>>) -> Self {
        let encryptor = Encryptor::new(key);

        Self { encryptor }
    }

    pub async fn connect(&mut self, ip: &str, port: u16, filepath: &Path) -> io::Result<()> {
        let mut sender = Sender::new(ip, port).await?;

        // Send the filename first
        let filename = filepath.file_name().unwrap().to_str().unwrap();
        let filename_len = filename.len() as u16;
        sender.send(&filename_len.to_be_bytes()).await?;
        sender.send(filename.as_bytes()).await?;

        // open the file
        let mut file = File::open(filepath).await?;

        // create a buffer to store chunks of the file
        let mut buffer = [0; 1024];

        // read the file chunk by chunk
        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            // encrypt the chunk
            let encrypted = self.encryptor.encrypt(&buffer[..n])?;

            sender.send_chunked(&encrypted, 1024).await?;
        }
        println!("{} has been sent", filepath.display());
        Ok(())
    }
}
