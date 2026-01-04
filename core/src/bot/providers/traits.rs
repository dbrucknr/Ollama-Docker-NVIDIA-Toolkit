// Standard Library Crates
use std::{convert::Infallible, sync::Arc};

// Third Party Library Crates
use axum::{
    Extension, Json,
    response::sse::{Event, Sse},
};
use futures::Stream;

// Local Library Crates
use crate::{
    api::{error::ControllerError, schemas::QueryRequestBody},
    bot::providers::{OllamaProvider, shimmy::ShimmyProvider},
};

// I think these traits may belong in /controllers - it feels like domain contamination
// to have providers define api specific contexts (Json, Sse, ControllerError)

pub trait GenerateResponse {
    fn generate_response(
        provider: Extension<Arc<ShimmyProvider>>,
        body: Json<QueryRequestBody>,
    ) -> impl Future<Output = Result<String, ControllerError>> + Send;
}

pub trait StreamResponse {
    fn stream_response(
        provider: Extension<Arc<ShimmyProvider>>,
        body: Json<QueryRequestBody>,
    ) -> impl Future<
        Output = Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send>, ControllerError>,
    >;
}
