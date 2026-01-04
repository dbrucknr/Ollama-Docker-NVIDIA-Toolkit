// Local Module Registry
pub mod api;
pub mod app;
pub mod bot;
pub mod server;

// Standard Library Crates
use std::{error::Error, io::Result as IoResult, process::ExitCode};

// Local Library Crates
use api::Api;
use app::Application;
use server::HttpServer;

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn Error>> {
    let api = Api::new()?;
    let app = Application::new(api);
    let server = HttpServer::new(app).await?;
    Ok(server.start().await?)
}
