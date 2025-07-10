use anyhow::{Context, Result};

pub struct AppConfig {
    pub llm: LlmConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let project_name = std::env::var("PROJECT_ID")
            .context("環境変数 PROJECT_ID が設定されていません")?;
        let model_id = std::env::var("MODEL_NAME")
            .context("環境変数 MODEL_NAME が設定されていません")?;

        let endpoint = format!(
            "https://aiplatform.googleapis.com/v1/projects/{}/locations/global/publishers/google/models/{}/:generateContent",
            project_name, model_id,
        );

        Ok(Self {
            llm: LlmConfig { endpoint },
        })
    }
}

pub struct LlmConfig {
    pub endpoint: String,
}
