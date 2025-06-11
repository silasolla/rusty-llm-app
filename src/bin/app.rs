use std::net::{Ipv4Addr, SocketAddr};
use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Router
use api::{
    handler::ping::PingApiDoc,
    route::ping::build_ping_routers,
};

// Documentation
#[derive(OpenApi)]
#[openapi(paths(), components(schemas()))]
struct ApiDoc;

// Create App
#[tokio::main]
async fn main() -> Result<()> {
    // Merge Docs
    let mut doc = ApiDoc::openapi();
    doc.merge(PingApiDoc::openapi());
    // Create Routes
    let app = Router::new()
        .merge(build_ping_routers())
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", doc));
    // Listen
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}
