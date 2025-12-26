use crate::protocol::Protocol;
use anyhow::Result;
use async_trait::async_trait;
use std::process::Command;

pub struct Smb {
    pub domain: Option<String>,
}

#[async_trait]
impl Protocol for Smb {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let username = if let Some(ref d) = self.domain {
            format!("{}\\{}", d, user)
        } else {
            user.to_string()
        };

        let output = Command::new("smbclient")
            .args([
                "-L",
                &format!("//{}", target),
                "-U",
                &format!("{}%{}", username, pass),
                "-m",
                "SMB2",
            ])
            .output()?;

        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(
            output.status.success()
                && !stderr.contains("NT_STATUS_LOGON_FAILURE")
                && !stderr.contains("NT_STATUS_ACCESS_DENIED"),
        )
    }
}
