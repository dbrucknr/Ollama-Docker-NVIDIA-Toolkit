use std::convert::Infallible;
use chrono::prelude::*;
use rig::{
    tool::Tool,
    completion::ToolDefinition,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct CurrentDateTool;

#[derive(Deserialize)]
pub struct CurrentDateToolArgs {
    format: Option<String>,
}

// Note - the time context in this tool isn't quite right yet. Seeing inconsistent results when using it in the /ui.
impl Tool for CurrentDateTool {
    const NAME: &'static str = "current_date";
    type Error = Infallible;
    type Args = CurrentDateToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "ONLY use this tool when the user explicitly asks for the current date or time. DO NOT use it otherwise.".to_string(),
            parameters: json!({
                "format": {
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string",
                            "description": "Optional date/time format string. Defaults to RFC3339."
                        }
                    },
                    "required": []
                }
            }),
        }
    }

    async fn call(
            &self,
            args: Self::Args,
        ) -> Result<Self::Output, Self::Error> {
            let now = Utc::now();
            println!("Current date and time: {}", now);

            let output = match args.format.as_deref() {
                Some(fmt) => now.format(fmt).to_string(),
                None => now.to_rfc3339(),
            };

            Ok(output)
    }
}
