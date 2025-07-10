use serde::Serialize;
use std::borrow::Cow;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PingResponse {
    #[schema(example = "Pong")]
    pub message: Cow<'static, str>,
}
