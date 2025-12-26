use crate::protocol::Protocol;
use anyhow::Result;
use async_trait::async_trait;
use std::process::Command;
use std::time::Duration;

pub struct WinRm {
    pub domain: Option<String>,
    pub ssl: bool,
}

#[async_trait]
impl Protocol for WinRm {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let mut cmd = Command::new("evil-winrm");

        cmd.args(["-i", target, "-u", user, "-p", pass]);

        if self.ssl {
            cmd.arg("-S");
        }

        if let Some(ref d) = self.domain {
            cmd.args(["-d", d]);
        }

        let output = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()?;

        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);

        Ok(
            out.contains("Evil-WinRM shell")
                && !err.contains("AuthorizationError")
                && !err.contains("WinRM::WinRMAuthorizationError"),
        )
    }
}
