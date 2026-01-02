// Local Module Registry
pub mod traits;

// Third Party Library Crates
use rig::agent::Agent;
use rig::providers::ollama::CompletionModel;
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
        // NOTE - mistral must be pulled into Ollama
        // docker exec -it ollama ollama run mistra
        let agent = Client::from_env()
            .agent("mistral")
            .preamble(
                r#"
                You are a helpful assistant that can solve problems. Use the tools provided to answer the user's question.
                "#,
            )
            .tool(CurrentDateTool)
            .tool(Adder)
            .tool(Subtract)
            .temperature(0.5)
            .build();

        Self { agent }
    }
}
