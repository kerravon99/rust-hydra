use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Protocol: Send + Sync {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool>;
}
