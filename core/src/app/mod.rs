// Local Module Registry
pub mod traits;

use std::path::Path;

// Third Party Library Crates
use axum::{Router, routing::get_service};
use tower_http::services::{ServeDir, ServeFile};

// Local Library Crates
use traits::ApplicationRouter;
use crate::api::traits::ApiRouter;

pub struct Application {
    router: Router,
}
impl Application {
    pub fn new(
        api: impl ApiRouter + 'static,
    ) -> Self {
        let assets = ServeDir::new("./static/assets");
        let index = ServeFile::new("./static/index.html");

        let assets_path = "./static/assets";
        println!(
            "Serving assets from {:?}, exists: {}",
            assets_path,
            Path::new(assets_path).exists()
        );

        Self {
            router: Router::new()
                .nest("/api", api.router())
                .nest_service("/assets", assets)
                .fallback_service(get_service(index))
        }
    }
}

impl ApplicationRouter for Application {
    fn router(self) -> Router {
        // I think App-level middleware can be registered here
        self.router
    }
}
