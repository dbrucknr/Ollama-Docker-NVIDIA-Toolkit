pub mod server;
pub mod app;
pub mod api;

use std::{process::ExitCode, io::Result as IoResult};

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
