use crate::model::llm::{PromptRequest, PromptResponse};
use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use std::borrow::Cow;
use utoipa::OpenApi;

use registry::AppRegistry;

#[derive(OpenApi)]
#[openapi(
    paths(prompt,),
    components(schemas(PromptRequest), schemas(PromptResponse),)
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
pub async fn prompt(
    State(registry): State<AppRegistry>,
    Json(req): Json<PromptRequest>,
) -> Result<Json<PromptResponse>, StatusCode> {
    // ここで Registry を経由して Gateway からリクエストする
    let res = registry
        .llm_gateway()
        .ask(&req.message)
        .await
        .map_err(|e| {
            eprintln!("LLM ask error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(PromptResponse {
        message: Cow::Owned(res),
    }))
}
