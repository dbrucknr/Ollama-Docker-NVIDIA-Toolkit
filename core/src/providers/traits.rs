use std::sync::Arc;
use axum::Extension;
use crate::{api::error::ControllerError, providers::OllamaProvider};

pub trait GenerateResponse {
    fn generate_response(
        provider: Extension<Arc<OllamaProvider>>
    ) -> impl Future<Output = Result<String, ControllerError>> + Send;
}
