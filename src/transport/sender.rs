// src/transport/sender.rs
use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream,
};

pub struct Sender {
    stream: TcpStream,
}

impl Sender {
    pub async fn new(ip: &str, port: u16) -> io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    pub async fn send_chunked(&mut self, data: &[u8], chunk_size: usize) -> io::Result<()> {
        for chunk in data.chunks(chunk_size) {
            self.stream.write_all(chunk).await?;
        }
        Ok(())
    }
}
