// src/main.rs
mod p2p;
mod security;
mod ui;

use structopt::StructOpt;
use ui::io::Opt;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let opt = Opt::from_args();

    p2p::discovery::discover_peer(opt.ip, opt.port, &opt.file).await?;

    Ok(())
}
