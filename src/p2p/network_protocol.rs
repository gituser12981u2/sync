// src/p2p/network_protocol.rs
use crate::peer:Peer;

pub struct NetworkProtocol;

impl NetworkProtocol {
    pub fn connect(peer: &Peer) -> tokio::io::Result<()> {
        peer.connect().await;
    }
}