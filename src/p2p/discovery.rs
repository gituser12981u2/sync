// src/p2p/discovery.rs
use crate::peer::Peer;
use crate::network_protocol::NetworkProtocol;

pub fn discover_peer(ip: String, port: u16) -> tokio::io::Result<()> {
    let peer = Peer::new(ip, port);
    NetworkProtocol::connect(&peer);
}
