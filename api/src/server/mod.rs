use std::{net::SocketAddr, process::ExitCode, io::Result as IoResult};
use axum::{serve, Router};
use tokio::net::TcpListener;

use crate::app::traits::ApplicationRouter;

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    pub async fn new(
        application: impl ApplicationRouter,
    ) -> IoResult<Self> {
        let address = SocketAddr::from(([0, 0, 0, 0], 8000));
        let listener = TcpListener::bind(address).await?;
        let router = application.router();
        Ok(Self { listener, router })
    }

    pub async fn start(self) -> IoResult<ExitCode> {
        serve(self.listener, self.router).await?;
        Ok(ExitCode::SUCCESS)
    }
}
