// src/p2p/peer.rs
use crate::security::psk::PskSession;
use rand::RngCore;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Peer {
    ip: String,
    port: u16,
    psk: Vec<u8>,
}

impl Peer {
    pub fn new(ip: String, port: u16) -> Self {
        let mut psk = vec![0u8; 32]; // 256 bits
        rand::thread_rng().fill_bytes(&mut psk);

        Self { ip, port, psk }
    }

    pub async fn connect(&mut self, filepath: &Path) -> io::Result<()> {
        let addr = format!("{}:{}", self.ip, self.port);

        // connect to the server
        let mut stream = TcpStream::connect(addr).await?;

        // print the PSK for this IP
        println!("PSK for IP {}: {}", self.ip, hex::encode(&self.psk));

        // create a PskSession for encryption and decryption
        let psk_session = PskSession::new(self.psk.clone());

        // send the filename
        let filename = filepath.file_name().unwrap().to_str().unwrap();

        let filename_bytes = filename.as_bytes();

        println!("file name after bytes: {}", filename);

        stream.write_u16(filename_bytes.len() as u16).await?;
        stream.write_all(filename_bytes).await?; // send 0 as a delimiter

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
            let encrypted = psk_session.encrypt(&buffer[..n])?;

            // send the chunk
            stream.write_all(&encrypted).await?;
        }
        println!("{} has been sent", filename);
        Ok(())
    }
}
