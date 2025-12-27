// Local Module Registry
pub mod traits;

// Third Party Library Crates
use axum::Router;

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
        Self {
            router: Router::new().nest("/api", api.router())
        }
    }
}

impl ApplicationRouter for Application {
    fn router(self) -> Router {
        // I think App-level middleware can be registered here
        self.router
    }
}
