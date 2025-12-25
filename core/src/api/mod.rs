use axum::{Router};

pub mod error;

pub mod controllers;
use controllers::OllamaController;

pub mod traits;
use traits::ApiRouter;

use crate::api::traits::ControllerRoutes;

pub struct Api {
    // Register controllers here
    router: Router,
    ollama: OllamaController,
}
impl Api {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            ollama: OllamaController::new(),
        }
    }
}

impl ApiRouter for Api {
    fn router(self) -> Router {
        // Add controller routes in this layer and /api specific middleware
        self.router.nest("/ollama", self.ollama.routes())
    }
}
