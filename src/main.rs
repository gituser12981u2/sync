// src/main.rs
mod p2p

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    p2p::discovery::discover_peer(ip, port)
}