use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum DetectedProtocol {
    Http,
    Https,
    Ssh,
    Ftp,
    Smb,
    WinRm,
    WinRmSsl,
}

fn port_open(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(sock) = addrs.next() {
            return TcpStream::connect_timeout(&sock, Duration::from_millis(800)).is_ok();
        }
    }
    false
}

pub fn detect(target: &str) -> Option<DetectedProtocol> {
    if target.contains("://") {
        let scheme = target.split("://").next().unwrap_or("");
        return match scheme {
            "http" => Some(DetectedProtocol::Http),
            "https" => Some(DetectedProtocol::Https),
            "ssh" => Some(DetectedProtocol::Ssh),
            "ftp" => Some(DetectedProtocol::Ftp),
            "smb" => Some(DetectedProtocol::Smb),
            "winrm" => Some(DetectedProtocol::WinRm),
            _ => None,
        };
    }

    if port_open(target, 445) {
        return Some(DetectedProtocol::Smb);
    }
    if port_open(target, 5985) {
        return Some(DetectedProtocol::WinRm);
    }
    if port_open(target, 5986) {
        return Some(DetectedProtocol::WinRmSsl);
    }
    if port_open(target, 22) {
        return Some(DetectedProtocol::Ssh);
    }
    if port_open(target, 21) {
        return Some(DetectedProtocol::Ftp);
    }
    if port_open(target, 80) {
        return Some(DetectedProtocol::Http);
    }
    if port_open(target, 443) {
        return Some(DetectedProtocol::Https);
    }

    None
}
