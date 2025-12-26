use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryRequestBody {
    pub query: String,
}
