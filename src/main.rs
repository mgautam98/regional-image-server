#![allow(dead_code)]

mod ip_finder;

use anyhow::Result;
use axum::{
    extract::{ConnectInfo, Extension},
    handler::{get, post, Handler},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, str::FromStr, time::Duration};
use tracing::{info, Level};

use std::convert::Infallible;
use tower::{BoxError, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

use crate::ip_finder::{ImageStore, IpFinder};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    App::new().run().await
}

struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn run(&self) -> Result<()> {
        let db = ImageStore::new(PathBuf::from_str("./images")?);
        let finder = IpFinder::new();

        let app = Router::new()
            .route("/save", post(save_img))
            .route("/:img", get(get_img))
            .layer(
                ServiceBuilder::new()
                    .timeout(Duration::from_secs(10))
                    .layer(TraceLayer::new_for_http())
                    .layer(AddExtensionLayer::new(db))
                    .layer(AddExtensionLayer::new(finder))
                    .into_inner(),
            )
            .handle_error(|error: BoxError| {
                let result = if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                };

                Ok::<_, Infallible>(result)
            })
            // Make sure all errors have been handled
            .check_infallible();

        // make sure this is added as the very last thing
        let app = app.or(handler_404.into_service());

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
            .await
            .unwrap();

        Ok(())
    }
}

async fn get_img(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(db): Extension<ImageStore>,
    Extension(finder): Extension<IpFinder>,
) -> String {
    let country_short = finder.find_country_short(addr.to_string().as_str()).await;
    if let Ok(img) = db.read_image("1.txt", country_short).await {
        return format!("{:?}", img);
    }
    format!("IP: {}\nNot Found {}", addr, "1.txt")
}

async fn save_img(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {}", addr)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT FOUND")
}
