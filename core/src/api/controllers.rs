use std::{convert::Infallible, sync::Arc};
use futures::{Stream, stream::StreamExt};
use axum::{Router, routing::post, Extension, Json, response::sse::{Event, Sse}};
use rig::{agent::MultiTurnStreamItem, completion::Prompt, streaming::{StreamedAssistantContent, StreamingPrompt}};

use crate::providers::{OllamaProvider, traits::{GenerateResponse, StreamResponse}};
use crate::api::{traits::ControllerRoutes, error::ControllerError, schemas::{QueryRequestBody, StreamingChunk}};

pub struct OllamaController {
    pub provider: Arc<OllamaProvider>,
}

impl OllamaController {
    pub fn new() -> Self {
        let ollama = OllamaProvider::new();
        Self { provider: Arc::new(ollama) }
    }
}
// POST http://localhost:8000/api/ollama/generate
impl GenerateResponse for OllamaController {
    async fn generate_response(
        Extension(provider): Extension<Arc<OllamaProvider>>,
        Json(body): Json<QueryRequestBody>
    ) -> Result<String, ControllerError> {
        let response = provider.agent
            .prompt(body.query)
            .await
            .map_err(|_| ControllerError::InternalServerError)?;

        Ok(response.to_string())
    }
}
// POST http://localhost:8000/api/ollama/stream
impl StreamResponse for OllamaController {
    async fn stream_response(
        Extension(provider): Extension<Arc<OllamaProvider>>,
        Json(body): Json<QueryRequestBody>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>> + Send> {

        let response = provider.agent.stream_prompt(body.query).await;

        // I think this needs a JSON response to help avoid some of the whitespace issues in /ui
        let stream = response.map(|chunk| {
            Ok(match chunk {
                Ok(reply_chunk) => match reply_chunk {

                    MultiTurnStreamItem::FinalResponse(response) => {
                        Event::default()
                            .event("final-response")
                            .data(response.response())
                    }

                    MultiTurnStreamItem::StreamAssistantItem(data_chunk) => match data_chunk {

                        StreamedAssistantContent::Text(text) => {
                            match Event::default()
                                .event("assistant")
                                .json_data(StreamingChunk::new(text.text)) {
                                Ok(event) => event,
                                // TODO - Trace this error
                                Err(_) =>
                                    Event::default().event("error").data("Could not create text event")
                                ,
                            }
                        }

                        StreamedAssistantContent::ToolCall(_) => {
                            Event::default()
                                .event("tool-call")
                                .data("tool call")
                        }

                        StreamedAssistantContent::ToolCallDelta { id, delta } => {
                            Event::default()
                                .event("tool-call-delta")
                                .data(delta)
                        }

                        StreamedAssistantContent::Reasoning(_) => {
                            Event::default()
                                .event("reasoning")
                                .data("reasoning")
                        }

                        StreamedAssistantContent::ReasoningDelta { id: _, reasoning } => {
                            Event::default()
                                .event("reasoning-delta")
                                .data(reasoning)
                        }

                        StreamedAssistantContent::Final(_) => {
                            Event::default()
                                .event("conclude")
                                // .data("")
                        }
                    },

                    MultiTurnStreamItem::StreamUserItem(_) => {
                        Event::default()
                            .event("user")
                            .data("user input")
                    }

                    _ => Event::default().data(""),
                },

                Err(_) => Event::default()
                    .event("error")
                    .data("stream error"),
            })
        });

        Sse::new(stream)
    }
}
impl ControllerRoutes for OllamaController {
    fn routes(self) -> Router {
        Router::new()
        .route("/stream", post(Self::stream_response))
        .route("/generate", post(Self::generate_response))
        .layer(Extension(self.provider))
    }
}
