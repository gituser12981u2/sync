// src/main.rs
mod data;
mod p2p;
mod security;
mod transport;
mod ui;

use p2p::peer::Peer;
use structopt::StructOpt;
use ui::io::Opt;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let opt = Opt::from_args();

    let mut peer = Peer::new(None);
    peer.connect(&opt.ip, opt.port, &opt.file).await?;

    Ok(())
}
