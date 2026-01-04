// Local Module Registry
pub mod controllers;
pub mod error;
pub mod schemas;
pub mod traits;

use std::error::Error;

// Third Party Library Crates
use axum::Router;

// Local Library Crates
use crate::api::traits::ControllerRoutes;
use controllers::ShimmyController;
use traits::ApiRouter;

pub struct Api {
    // Register controllers here
    router: Router,
    ollama: ShimmyController, // Consider dynamic dispatch pattern for testability
}
impl Api {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let shimmy = ShimmyController::new()?;
        Ok(Self {
            router: Router::new(),
            ollama: shimmy,
        })
    }
}

impl ApiRouter for Api {
    fn router(self) -> Router {
        // Add controller routes in this layer and /api specific middleware
        self.router.nest("/ollama", self.ollama.routes())
    }
}
