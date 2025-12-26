use crate::protocol::Protocol;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

pub struct HttpForm {
    pub user_field: String,
    pub pass_field: String,
    pub fail: String,
}

#[async_trait]
impl Protocol for HttpForm {
    async fn authenticate(&self, target: &str, user: &str, pass: &str) -> Result<bool> {
        let client = Client::new();
        let res = client
            .post(target)
            .form(&[
                (&self.user_field, user),
                (&self.pass_field, pass),
            ])
            .send()
            .await?
            .text()
            .await?;

        Ok(!res.contains(&self.fail))
    }
}
