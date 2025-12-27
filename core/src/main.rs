// Local Module Registry
pub mod server;
pub mod app;
pub mod api;
pub mod providers;

// Standard Library Crates
use std::{process::ExitCode, io::Result as IoResult};

// Local Library Crates
use api::Api;
use app::Application;
use server::HttpServer;

#[tokio::main]
async fn main() -> IoResult<ExitCode> {
    let api = Api::new();
    let app = Application::new(api);
    let server = HttpServer::new(app).await?;
    Ok(server.start().await?)
}
