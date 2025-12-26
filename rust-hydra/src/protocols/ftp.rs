use crate::protocol::Protocol;
use anyhow::Result;
use async_trait::async_trait;
use ftp::FtpStream;

pub struct Ftp;

#[async_trait]
impl Protocol for Ftp {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let mut ftp = FtpStream::connect(format!("{}:21", target))?;
        Ok(ftp.login(user, pass).is_ok())
    }
}
