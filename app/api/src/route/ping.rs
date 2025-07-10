use crate::handler::ping::ping;
use axum::{Router, routing::get};

use registry::AppRegistry;

pub fn build_ping_routers() -> Router<AppRegistry> {
    let routes = Router::new().route("/", get(ping));
    Router::new().nest("/ping", routes)
}
