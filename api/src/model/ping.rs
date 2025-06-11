use std::borrow::Cow;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PingResponse {
    #[schema(example = "Pong")]
    pub message: Cow<'static, str>,
}
