use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;
use std::convert::Infallible;
use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Deserialize)]
pub struct WhoisArgs {
    domain: String,
}

// #[derive(Serialize)]
// struct WhoisRecord {
//     raw: String,
//     fields: std::collections::HashMap<String, String>,
// }

async fn whois(query: &str, server: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect((server, 43)).await?;
    stream.write_all(format!("{query}\r\n").as_bytes()).await?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

pub struct WhoisTool;

impl Tool for WhoisTool {
    const NAME: &'static str = "whois";
    type Error = Infallible;
    type Args = WhoisArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "whois".to_string(),
            description: "Discover who a domain belongs to".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "domain": {
                        "type": "string",
                        "description": "The domain to search"
                    }
                },
                "required": ["domain"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match whois(&args.domain, "whois.iana.org").await {
            Ok(text) => text,
            Err(err) => format!("WHOIS lookup failed for {}: {}", args.domain, err),
        };
        Ok(result)
    }
}
