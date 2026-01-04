// Standard Library Crates
use std::{convert::Infallible, error::Error, sync::Arc};

// Third Party Library Crates
use axum::{
    Extension, Json, Router,
    response::sse::{Event, Sse},
    routing::post,
};
use futures::{Stream, stream::StreamExt};
use rig::{
    agent::MultiTurnStreamItem,
    streaming::{StreamedAssistantContent, StreamingCompletion, StreamingPrompt},
};

// Local Library Crates
use crate::api::{
    error::ControllerError,
    schemas::{QueryRequestBody, StreamingChunk},
    traits::ControllerRoutes,
};
use crate::bot::providers::{shimmy::ShimmyProvider, traits::StreamResponse};

pub struct ShimmyController {
    pub provider: Arc<ShimmyProvider>,
}
impl ShimmyController {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let shimmy = ShimmyProvider::new()?;
        Ok(Self {
            provider: Arc::new(shimmy),
        })
    }
}
// POST http://localhost:8000/api/ollama/stream
impl StreamResponse for ShimmyController {
    async fn stream_response(
        Extension(provider): Extension<Arc<ShimmyProvider>>,
        Json(body): Json<QueryRequestBody>,
    ) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send>, ControllerError> {
        // let response = provider.agent.stream_prompt(body.query).await;
        // println!("Sending prompt to Shimmy: {}", body.query);
        // println!("Using model: {}", provider.agent.);
        // println!("Base URL: {}", provider.agent);
        let response = provider
            .agent
            .stream_completion(body.query, Vec::new())
            .await
            .map_err(|_| ControllerError::InternalServerError)?;

        // let data = response
        //     .stream()
        //     .await
        //     .map_err(|_| ControllerError::InternalServerError)?;
        let stream = response
            .stream()
            .await
            .map_err(|_| ControllerError::InternalServerError)?
            .map(|chunk| {
                Ok(match chunk {
                    Ok(reply_chunk) => match reply_chunk {
                        StreamedAssistantContent::Text(text) => {
                            match Event::default()
                                .event("assistant")
                                .json_data(StreamingChunk::new(text.text))
                            {
                                Ok(event) => event,
                                // TODO - Trace this error
                                Err(_) => Event::default()
                                    .event("error")
                                    .data("Could not create text event"),
                            }
                        }
                        StreamedAssistantContent::ToolCall(..) => {
                            Event::default().event("tool-call").data("tool")
                        }
                        StreamedAssistantContent::ToolCallDelta { id, delta } => {
                            Event::default().event("tool-call-delta").data(delta)
                        }
                        StreamedAssistantContent::Reasoning(_) => {
                            Event::default().event("reasoning").data("reasoning")
                        }
                        StreamedAssistantContent::ReasoningDelta { id: _, reasoning } => {
                            Event::default().event("reasoning-delta").data(reasoning)
                        }
                        StreamedAssistantContent::Final(_) => {
                            Event::default().event("final-response").data("")
                        }
                    },
                    Err(error) => {
                        println!("Stream Error: {error:#?}");
                        Event::default().event("error").data("stream error")
                    }
                })
            });

        // I think this needs a JSON response to help avoid some of the whitespace issues in /ui
        // let stream = response.map(|chunk| {
        //     Ok(match chunk {
        //         Ok(reply_chunk) => match reply_chunk {
        //             MultiTurnStreamItem::FinalResponse(response) => Event::default()
        //                 .event("final-response")
        //                 .data(response.response()),

        //             MultiTurnStreamItem::StreamAssistantItem(data_chunk) => match data_chunk {
        //                 StreamedAssistantContent::Text(text) => {
        //                     match Event::default()
        //                         .event("assistant")
        //                         .json_data(StreamingChunk::new(text.text))
        //                     {
        //                         Ok(event) => event,
        //                         // TODO - Trace this error
        //                         Err(_) => Event::default()
        //                             .event("error")
        //                             .data("Could not create text event"),
        //                     }
        //                 }

        //                 StreamedAssistantContent::ToolCall(call) => {
        //                     println!("Tool Call -----> {:#?}", call);

        //                     Event::default().event("tool-call").data(call.function.name)
        //                 }

        //                 StreamedAssistantContent::ToolCallDelta { id, delta } => {
        //                     println!("{} - {:#?}", id, delta);
        //                     Event::default().event("tool-call-delta").data(delta)
        //                 }

        //                 StreamedAssistantContent::Reasoning(_) => {
        //                     Event::default().event("reasoning").data("reasoning")
        //                 }

        //                 StreamedAssistantContent::ReasoningDelta { id: _, reasoning } => {
        //                     Event::default().event("reasoning-delta").data(reasoning)
        //                 }

        //                 StreamedAssistantContent::Final(_) => {
        //                     Event::default().event("conclude")
        //                     // .data("")
        //                 }
        //             },

        //             MultiTurnStreamItem::StreamUserItem(_) => {
        //                 Event::default().event("user").data("user input")
        //             }

        //             _ => Event::default().data(""),
        //         },

        //         Err(error) => {
        //             println!("Stream Error: {error:?}");
        //             Event::default().event("error").data("stream error")
        //         }
        //     })
        // });

        Ok(Sse::new(stream))
    }
}
impl ControllerRoutes for ShimmyController {
    fn routes(self) -> Router {
        Router::new()
            .route("/stream", post(Self::stream_response))
            // .route("/generate", post(Self::generate_response))
            .layer(Extension(self.provider))
    }
}
