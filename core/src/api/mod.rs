// Local Module Registry
pub mod error;
pub mod traits;
pub mod schemas;
pub mod controllers;

// Third Party Library Crates
use axum::{Router};

// Local Library Crates
use traits::ApiRouter;
use controllers::OllamaController;
use crate::api::traits::ControllerRoutes;

pub struct Api {
    // Register controllers here
    router: Router,
    ollama: OllamaController, // Consider dynamic dispatch pattern for testability
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
