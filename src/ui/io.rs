// src/io.rs
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sync")]
pub struct Opt {
    // the ip address to connect to
    #[structopt(short = "i", long = "ip", default_value = "127.0.0.1")]
    pub ip: String,

    // the port to connect to
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    // the file to send
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub file: PathBuf,

    // command to get the PSK for a specific contact
    #[structopt(short = "c", long = "contact")]
    pub contact: Option<String>,

    // command to save the contact
    #[structopt(short = "s", long = "save")]
    pub save_contact: bool,

    // command to set manual mode
    #[structopt(short = "m", long = "manual")]
    pub manual_mode: bool,

    // command to get the PSK for a specific IP
    #[structopt(long = "get-psk")]
    pub get_psk: Option<String>,

    // the contact name to save or retrieve
    #[structopt(short = "n", long = "name")]
    pub name: Option<String>,
}
