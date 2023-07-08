// src/p2p/network_protocol.rs
use crate::p2p::peer::Peer;
use std::path::Path;

pub struct NetworkProtocol;

impl NetworkProtocol {
    pub async fn connect(peer: &mut Peer, filepath: &Path) -> tokio::io::Result<()> {
        peer.connect(filepath).await
    }
}
