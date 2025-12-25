use axum::Router;

pub trait ApplicationRouter {
    fn router(self) -> Router;
}
