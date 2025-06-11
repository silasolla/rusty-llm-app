use axum::{routing::post, Router};
use crate::handler::llm::prompt;

pub fn build_llm_routers() -> Router {
    let routes = Router::new()
        .route("/prompt", post(prompt));
    Router::new()
        .nest("/llm", routes)
}
