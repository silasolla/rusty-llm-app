use crate::model::ping::PingResponse;
use axum::Json;
use std::borrow::Cow;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(ping,), components(schemas(PingResponse),))]
pub struct PingApiDoc;

#[utoipa::path(
    get,
    path = "/ping",
    description = "Sending a request to this endpoint returns a \"Pong\" message.",
    responses(
        (status = 200, description = "Your request was successfully submitted.", body = PingResponse),
    ),
)]
#[allow(clippy::unused_async)]
pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: Cow::Borrowed("Pong"),
    })
}
