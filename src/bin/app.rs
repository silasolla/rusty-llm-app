use std::net::{Ipv4Addr, SocketAddr};
use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use utoipa::{OpenApi, openapi::OpenApi as UtoipaOpenApi};
use utoipa_swagger_ui::SwaggerUi;

// Api
use api::{
    // Docs
    handler::ping::PingApiDoc,
    handler::llm::LlmApiDoc,
    // Routes
    route::ping::build_ping_routers,
    route::llm::build_llm_routers,
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
    // Create Routes
    let app = Router::new()
        .merge(build_ping_routers())
        .merge(build_llm_routers())
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", build_api_docs()));
    // Listen
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}
