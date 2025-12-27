// Third Party Library Crates
use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub enum ControllerError {
    InternalServerError,
}
impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        let (status, _, message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("INTERNAL_SERVER_ERROR"),
                String::from("An unexpected error occurred"),
            ),
        };
        (status, message).into_response()
    }
}
