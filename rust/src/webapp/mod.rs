use std::{
    net::SocketAddr, 
    path::PathBuf
};
use anyhow::{Context, Result};

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use axum::{
    response::{Html, IntoResponse},
    routing::{any, get},
    Router,
};
use tower_http::services::ServeDir;
use axum_server::{
    Handle,
    tls_rustls::RustlsConfig
};

// use crate::lights::LightsRemote;

mod redirect;
pub use redirect::redirect_http_to_https;

mod shutdown;
pub use shutdown::shutdown_signal;

// mod websocket;
// pub use websocket::ws_handler;

/// The path to the index file
const INDEX_FILE: &'static str = if cfg!(debug_assertions) {
    "../frontend/dist/index.html"
} else {
    "www/index.html"
};

/// The path to the secrets directory
const SECRETS_DIR: &'static str = if cfg!(debug_assertions) {
    "../secrets"
} else {
    "secrets"
};

/// The path to the assets directory
const ASSETS_DIR: &'static str = if cfg!(debug_assertions) {
    "../frontend/public/assets"
} else {
    "www/assets"
};

/// start the web-app by starting the web-server
pub async fn start(handle: Handle) -> Result<()> {
    info!("Starting the lights controller web-server");

    // set up the TLS config
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(SECRETS_DIR).join("lights.crt"),
        PathBuf::from(SECRETS_DIR).join("lights.key"),
    )
    .await
    .context("Failed to load TLS config")?;

    let app = build().await?;

    let serv_addr_str = redirect::ADDR.to_string() + ":" + redirect::HTTPS_PORT;
    let serv_addr: SocketAddr = serv_addr_str.parse().context("Could not make a socket address from strings")?;
    axum_server::bind_rustls(serv_addr, config)
    .handle(handle)
    .serve(app.into_make_service())
    .await.context("Error starting web-server")?;

    Ok(())
}

pub async fn build() -> Result<Router> {
    debug!("Building Axum web-app objects");

    let static_index = tokio::fs::read_to_string(INDEX_FILE).await
    .context("Failed to read index.html")?;

    // build our application
    Ok(Router::new()
        // static files
        .nest_service("/assets", ServeDir::new(ASSETS_DIR))
        // default route to server main page:
        .route("/", get(|| async move { 
            // read in the html for a single page applications
            let index = tokio::fs::read_to_string(INDEX_FILE).await
            .unwrap_or(static_index);
            Html(index).into_response()
        }))
        // websocket handler
        // .route("/ws", any(ws_handler))
    )
}
