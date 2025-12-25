use axum::Router;

pub trait ApiRouter {
    fn router(&self) -> Router;
}
