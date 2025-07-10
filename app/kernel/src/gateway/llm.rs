use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait LlmGateway: Send + Sync {
    async fn ask(&self, prompt: &str) -> Result<String>;
    async fn fetch_token(&self) -> Result<String>;
}
