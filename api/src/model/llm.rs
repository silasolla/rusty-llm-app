use std::borrow::Cow;
use serde::{Serialize,Deserialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PromptRequest {
    #[schema(example = "Hello!")]
    pub message: Cow<'static, str>,
}

#[derive(Serialize, ToSchema)]
pub struct PromptResponse {
    #[schema(example = "Hello!")]
    pub message: Cow<'static, str>,
}
