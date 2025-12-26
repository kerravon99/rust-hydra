mod autodetect;
mod cli;
mod engine;
mod protocol;
mod protocols;
mod resume;
mod stream;

use autodetect::DetectedProtocol;
use clap::Parser;
use colored::*;
use std::sync::Arc;

use protocols::{
    ftp::Ftp,
    http::HttpForm,
    smb::Smb,
    ssh::Ssh,
    winrm::WinRm,
};

#[tokio::main]
async fn main() {
    println!("{}", "Rust-Hydra | AUTHORIZED LABS ONLY".red());

    let mut args = cli::Args::parse();

    if args.protocol == "auto" {
        match autodetect::detect(&args.target) {
            Some(DetectedProtocol::Smb) => args.protocol = "smb".into(),
            Some(DetectedProtocol::WinRm) => args.protocol = "winrm".into(),
            Some(DetectedProtocol::WinRmSsl) => {
                args.protocol = "winrm".into();
                args.ssl = true;
            }
            Some(DetectedProtocol::Ssh) => args.protocol = "ssh".into(),
            Some(DetectedProtocol::Ftp) => args.protocol = "ftp".into(),
            Some(DetectedProtocol::Http) => args.protocol = "http".into(),
            _ => {
                eprintln!("[-] Could not autodetect protocol");
                return;
            }
        }
    }

    let users = stream::load_users(&args.users);

    let proto: Arc<dyn protocol::Protocol> = match args.protocol.as_str() {
        "http" => Arc::new(HttpForm {
            user_field: args.user_field.clone().unwrap(),
            pass_field: args.pass_field.clone().unwrap(),
            fail: args.fail.clone().unwrap(),
        }),
        "ssh" => Arc::new(Ssh),
        "ftp" => Arc::new(Ftp),
        "smb" => Arc::new(Smb {
            domain: args.domain.clone(),
        }),
        "winrm" => Arc::new(WinRm {
            domain: args.domain.clone(),
            ssl: args.ssl,
        }),
        _ => panic!("Unsupported protocol"),
    };

    engine::run(proto, args, users).await;
}
