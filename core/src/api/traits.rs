use axum::Router;

// Maybe I can combine these traits into one?

pub trait ApiRouter {
    fn router(self) -> Router;
}

pub trait ControllerRoutes {
    fn routes(self) -> Router;
}
