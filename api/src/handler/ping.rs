use std::borrow::Cow;
use axum::Json;
use utoipa::OpenApi;
use crate::model::ping::PingResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        ping,
    ),
    components(
        schemas(PingResponse),
    )
)]
pub struct PingApiDoc;

#[utoipa::path(
    get,
    path = "/ping",
    description = "Ping",
    responses(
        (status = 200, description = "Pong", body = PingResponse),
    ),
)]
#[allow(clippy::unused_async)]
pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: Cow::Borrowed("Pong"),
    })
}
