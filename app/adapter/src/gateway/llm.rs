use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize};
use anyhow::{Result, Context};
use derive_new::new;
use serde_json;

use kernel::gateway::llm::LlmGateway;

#[derive(new)]
pub struct LlmGatewayImpl {
    client: Client,
    endpoint: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

#[async_trait]
impl LlmGateway for LlmGatewayImpl {
    async fn ask(&self, prompt: &str) -> Result<String> {
        eprintln!("🔑 Fetching token...");
        let token = self.fetch_token().await?;
        eprintln!("✅ Got token: {}", &token);

        eprintln!("🧠 Sending prompt...");
        let res = self.client.post(&self.endpoint)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "contents": [
                    {
                        "role": "user",
                        "parts": [
                            {
                                "text": prompt,
                            }
                        ]
                    }
                ]
            }))
            .send()
            .await
            .context("Failed to send LLM request")?;

        let status = res.status();
        eprintln!("📦 Response status: {}", status);

        let raw = res.text().await.context("Failed to read body")?;
        let parsed: GenerateContentResponse =
            serde_json::from_str(&raw).context("Failed to parse JSON")?;

        let answer = parsed
            .candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| p.text.trim_end())
            .unwrap_or("<no text returned>")
            .to_owned();

        Ok(answer)
    }

    async fn fetch_token(&self) -> Result<String> {
        let res = self.client
            .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .context("Failed to fetch token from metadata server")?;

        let token: TokenResponse = res
            .json()
            .await
            .context("Failed to parse token response")?;

        Ok(token.access_token)
    }
}
