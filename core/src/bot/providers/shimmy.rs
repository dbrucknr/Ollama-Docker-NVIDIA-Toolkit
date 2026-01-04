use std::error::Error;

use reqwest::header::{CONTENT_TYPE, HeaderValue};
use rig::{
    agent::Agent,
    prelude::*,
    providers::openai::{Client as OpenAI, CompletionModel},
};

// Im missing headers `Content-Type: application/json` when communicating with Shimmy...
// https://docs.rig.rs/guides/extension/write_your_own_provider
// https://github.com/joshua-mo-143/rig-custom-provider-example/blob/main/src/lib.rs

pub struct ShimmyProvider {
    pub agent: Agent<CompletionModel>,
}
impl ShimmyProvider {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = OpenAI::builder()
            .api_key("shimmy") // dummy key, required by Rig
            .base_url("http://shimmy-server:11435/v1") // Shimmy
            .build()?;

        // client.headers().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // I'm missing application/json header
        let agent = client
            .completion_model("model")
            .completions_api()
            .into_agent_builder()
            .preamble("You are a helpful assistant")
            .build();

        // let agent = client
        //     .completion_model("model")
        //     .preamble("Be precise and concise.")
        //     .temperature(0.5)
        //     .build();

        Ok(Self { agent })
    }
}
