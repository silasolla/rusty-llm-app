use std::sync::Arc;
use reqwest::Client;

use adapter::{gateway::llm::LlmGatewayImpl};
use kernel::gateway::llm::LlmGateway;
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    llm_gateway: Arc<dyn LlmGateway>,
}

impl AppRegistry {
    pub fn new(
        http_client: Arc<Client>,
        app_config: AppConfig,
    ) -> Self {
        let llm_gateway = Arc::new(LlmGatewayImpl::new(
            http_client.as_ref().clone(),
            app_config.llm.endpoint,
        ));
        Self {
            llm_gateway,
        }
    }

    pub fn llm_gateway(&self) -> Arc<dyn LlmGateway> {
        self.llm_gateway.clone()
    }
}
