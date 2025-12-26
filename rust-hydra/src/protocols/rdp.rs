use crate::protocol::Protocol;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rdp::client::Connector;
use rdp::core::event::RdpEvent;
use rdp::model::link::LinkState;
use std::net::TcpStream;
use std::time::Duration;

pub struct Rdp;

#[async_trait]
impl Protocol for Rdp {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let addr = format!("{}:3389", target);

        let stream = TcpStream::connect_timeout(
            &addr.parse()?,
            Duration::from_secs(5),
        )?;

        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        let mut connector = Connector::new()
            .with_username(user)
            .with_password(pass)
            .with_domain("") // domain optional; THM usually empty
            .build()?;

        let mut rdp = connector.connect(stream)?;

        loop {
            match rdp.read()? {
                RdpEvent::LinkStateChanged { state } => {
                    match state {
                        LinkState::Connected => {
                            // ✅ AUTH SUCCESS
                            return Ok(true);
                        }
                        LinkState::Failed => {
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
