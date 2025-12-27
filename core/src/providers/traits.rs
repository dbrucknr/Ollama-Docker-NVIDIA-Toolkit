// Standard Library Crates
use std::{convert::Infallible, sync::Arc};

// Third Party Library Crates
use futures::Stream;
use axum::{Extension, Json, response::sse::{Sse, Event}};

// Local Library Crates
use crate::{api::{error::ControllerError, schemas::QueryRequestBody}, providers::OllamaProvider};

// I think these traits may belong in /controllers - it feels like domain contamination
// to have providers define api specific contexts (Json, Sse, ControllerError)

pub trait GenerateResponse {
    fn generate_response(
        provider: Extension<Arc<OllamaProvider>>,
        body: Json<QueryRequestBody>
    ) -> impl Future<Output = Result<String, ControllerError>> + Send;
}

pub trait StreamResponse {
    fn stream_response(
        provider: Extension<Arc<OllamaProvider>>,
        body: Json<QueryRequestBody>
    ) -> impl Future<Output = Sse<impl Stream<Item = Result<Event, Infallible>> + Send>>;
}
