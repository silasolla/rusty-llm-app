use anyhow::Result;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use utoipa::{OpenApi, openapi::OpenApi as UtoipaOpenApi};
use utoipa_swagger_ui::SwaggerUi;
use reqwest::Client as HttpClient;

use shared::config::AppConfig;
use registry::AppRegistry;

// Api
use api::{
    handler::llm::LlmApiDoc,
    // Docs
    handler::ping::PingApiDoc,
    route::llm::build_llm_routers,
    // Routes
    route::ping::build_ping_routers,
};

// Documentation
#[derive(OpenApi)]
#[openapi(paths(), components(schemas()))]
struct ApiDoc;

// Merge Docs
fn build_api_docs() -> UtoipaOpenApi {
    let mut doc = ApiDoc::openapi();
    doc.merge(PingApiDoc::openapi());
    doc.merge(LlmApiDoc::openapi());
    doc
}

// Create App
#[tokio::main]
async fn main() -> Result<()> {
    // Registry
    let http_client = HttpClient::new();
    let app_config = AppConfig::new()?;
    let registry = AppRegistry::new(http_client.into(), app_config);

    // Create Routes
    let app = Router::new()
        .merge(build_ping_routers())
        .merge(build_llm_routers())
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", build_api_docs()))
        .with_state(registry);

    // Listen
    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}
