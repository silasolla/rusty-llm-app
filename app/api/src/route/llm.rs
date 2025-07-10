use crate::handler::llm::prompt;
use axum::{Router, routing::post};
use registry::AppRegistry;

pub fn build_llm_routers() -> Router<AppRegistry> {
    let routes = Router::new().route("/prompt", post(prompt));
    Router::new().nest("/llm", routes)
}
