// Third Party Library Crates
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryRequestBody {
    pub query: String,
}

#[derive(Serialize)]
pub struct StreamingChunk {
    pub content: String,
}
impl StreamingChunk {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
