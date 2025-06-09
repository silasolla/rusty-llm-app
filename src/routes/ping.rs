use std::borrow::Cow;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PingResponse {
    #[schema(example = "Pong")]
    pub message: Cow<'static, str>,
}

#[utoipa::path(
    get,
    path = "/ping",
    description = "Ping",
    responses(
        (status = 200, description = "Pong", body = PingResponse)
    ),
)]
#[allow(clippy::unused_async)]
pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: Cow::Borrowed("Pong"),
    })
}
