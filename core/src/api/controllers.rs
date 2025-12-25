use std::sync::Arc;
use axum::{Router, routing::get, Extension};
use rig::completion::{Prompt};

use crate::providers::{OllamaProvider, traits::GenerateResponse};
use crate::api::traits::ControllerRoutes;
use crate::api::error::ControllerError;

pub struct OllamaController {
    pub provider: Arc<OllamaProvider>,
}

impl OllamaController {
    pub fn new() -> Self {
        let provider = Arc::new(OllamaProvider::new());
        Self { provider }
    }
}
// GET http://localhost:8000/api/ollama/generate
impl GenerateResponse for OllamaController {
    async fn generate_response(
        Extension(provider): Extension<Arc<OllamaProvider>>
    ) -> Result<String, ControllerError> {
        let response = provider.agent
            .prompt("Hi there!")
            .await
            .map_err(|_| ControllerError::InternalServerError)?;

        Ok(response.to_string())
    }
}
impl ControllerRoutes for OllamaController {
    fn routes(self) -> Router {
        Router::new()
        .route("/generate", get(Self::generate_response))
        .layer(Extension(self.provider))
    }
}
