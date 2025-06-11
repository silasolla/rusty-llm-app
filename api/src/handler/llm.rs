use std::borrow::Cow;
use axum::Json;
use utoipa::OpenApi;
use crate::model::llm::{PromptRequest, PromptResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        prompt,
    ),
    components(
        schemas(PromptRequest),
        schemas(PromptResponse),
    )
)]
pub struct LlmApiDoc;

#[utoipa::path(
    post,
    path = "/llm/prompt",
    description = "Sending a request to this endpoint returns an AI-generated message.",
    request_body = PromptRequest,
    responses(
        (status = 200, description = "Your request was successfully submitted.", body = PromptResponse),
    ),
)]
#[allow(clippy::unused_async)]
pub async fn prompt(Json(_req): Json<PromptRequest>) -> Json<PromptResponse> {
    // ここで Registry を経由して Gateway からリクエストする
    Json(PromptResponse {
        message: Cow::Borrowed("Hello!"),
    })
}
