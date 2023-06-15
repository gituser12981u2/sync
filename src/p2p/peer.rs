// src/p2p/peer.rs
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt};

pub struct Peer {
    ip: String,
    port: u16,
}

impl Peer {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn connect(%self) {
        let addr = format!("{}:{}", self.ip, self.port);
        let mut stream = TcpStream::connect(&addr).await?;

        // send some data
        stream.write_all(b"Hello, World!").await?;

        Ok(())
    }
}