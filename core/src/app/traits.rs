// Third Party Library Crates
use axum::Router;

pub trait ApplicationRouter {
    fn router(self) -> Router;
}
