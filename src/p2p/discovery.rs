use crate::p2p::network_protocol::NetworkProtocol;
use crate::p2p::peer::Peer;
use std::path::Path;

pub async fn discover_peer(ip: String, port: u16, filepath: &Path) -> tokio::io::Result<()> {
    let mut peer = Peer::new(ip, port);
    NetworkProtocol::connect(&mut peer, filepath).await
}
