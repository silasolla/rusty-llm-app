use std::net::{Ipv4Addr, SocketAddr};
use anyhow::Result;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
// use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Router
mod routes;
use routes::ping::{ping, PingResponse};

fn create_app() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", ApiDoc::openapi()))
}

// Documentation
#[derive(OpenApi)]
#[openapi(paths(routes::ping::ping), components(schemas(PingResponse)))]
struct ApiDoc;

// Create App
#[tokio::main]
async fn main() -> Result<()> {
    let app = create_app();
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}
