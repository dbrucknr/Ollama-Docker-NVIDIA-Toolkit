// Local Module Registry
pub mod shimmy;
pub mod traits;

// Third Party Library Crates
use rig::agent::Agent;
// use rig::providers::openai::CompletionModel;
use rig::providers::ollama::CompletionModel;
// use rig::providers::openai::Client;
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::ollama::Client,
};

// Local Library Crates
use crate::bot::tools::{
    current_date::CurrentDateTool,
    math::{Adder, Subtract},
};

pub struct OllamaProvider {
    pub agent: Agent<CompletionModel>,
}

impl OllamaProvider {
    pub fn new() -> Self {
        let agent = Client::from_env()
            .agent("llama3.1")
            .preamble(
                r#"
                You are a helpful assistant that can solve problems. Use the tools provided to answer the user's question.
                "#,
            )
            // When using Ollama - these tools are not being assigned an ID
            .tool(CurrentDateTool)
            .tool(Adder)
            .tool(Subtract)
            .temperature(0.5)
            .build();

        Self { agent }
    }
}
