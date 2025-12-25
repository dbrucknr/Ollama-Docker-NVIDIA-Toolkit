use axum::{Router, routing::get};

pub mod traits;
use traits::ApiRouter;

pub struct Api {
    // Register controllers here
}
impl Api {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApiRouter for Api {
    fn router(&self) -> Router {
        // Add controller routes in this layer
        Router::new().route("/", get(|| async { "Hello, World!" }))
    }
}
