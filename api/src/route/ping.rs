use axum::{routing::get, Router};
use crate::handler::ping::ping;

pub fn build_ping_routers() -> Router {
    let routes = Router::new()
        .route("/", get(ping));
    Router::new()
        .nest("/ping", routes)
}
