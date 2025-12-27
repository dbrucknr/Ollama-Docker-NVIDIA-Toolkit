// Local Module Registry
pub mod traits;
// Third Party Library Crates
use rig::agent::Agent;
use rig::providers::ollama::CompletionModel;
use rig::{client::{CompletionClient, ProviderClient}, providers::ollama::Client};

pub struct OllamaProvider {
    pub agent: Agent<CompletionModel>,
}

impl OllamaProvider {
    pub fn new() -> Self {
        // NOTE - mistral must be pulled into Ollama
        // docker exec -it ollama ollama run mistra
        let agent = Client::from_env()
            .agent("mistral")
            .preamble("You're a friendly companion.")
            .temperature(0.5)
            .build();

        Self { agent }
    }
}
