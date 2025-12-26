use crate::protocol::Protocol;
use anyhow::Result;
use async_trait::async_trait;
use ssh2::Session;
use std::net::TcpStream;

pub struct Ssh;

#[async_trait]
impl Protocol for Ssh {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let tcp = TcpStream::connect(format!("{}:22", target))?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        Ok(sess.userauth_password(user, pass).is_ok())
    }
}
