mod ip_finder;

use anyhow::Result;
use axum::{handler::get, Router, extract::ConnectInfo};
use std::net::SocketAddr;
use tracing::{debug, Level};

use crate::ip_finder::{IpFinder, ImageStore};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    App::new().run().await
}

struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn run(&self) -> Result<()> {
        let app = Router::new().route("/", get(handler)).route("/so", get(handler));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(
                app.into_make_service_with_connect_info::<SocketAddr, _>()
            )
            .await
            .unwrap();

        Ok(())
    }
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {}", addr)
}