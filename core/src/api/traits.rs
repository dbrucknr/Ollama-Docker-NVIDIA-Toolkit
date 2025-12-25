use axum::Router;

pub trait ApiRouter {
    fn router(self) -> Router;
}

pub trait ControllerRoutes {
    fn routes(self) -> Router;
}
